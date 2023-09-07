use std::cmp;

use gloo::events::EventListener;
use leptos::logging::log;
use leptos::*;

use super::session::PCSession;
use super::PC;
use crate::buffs::BuffProp;
use crate::items::simple;
use crate::items::simple::meta::FATIGUE;
use crate::pc::session::SlotRange;
use crate::utils::index_map::IndexMap;
use crate::utils::turns::TURNS_IN_DAY;
use crate::utils::RwProvided;
use crate::views::revealer::Revealer;
use crate::views::toast::Toast;

pub(super) fn on_rally() {
    PC::update(|pc| {
        add_fatigue(pc);
        restore_buffs(pc, |prop| matches!(prop, BuffProp::Rally));
    })
}

fn add_fatigue(pc: &mut PC) {
    Toast::show("rally", "fatigue has been added to inventory");
    pc.inventory.add(FATIGUE.into());
}

pub(super) fn on_rest(days: u64, heal_health: bool) {
    PC::update(|pc| {
        let mut hp_gain = "no health restored".to_string();
        if heal_health {
            let heal = cmp::min(days as i32, pc.health_dmg);
            pc.health_dmg -= heal;
            hp_gain = format!("+{heal} health");
        }
        pc.turns.0 += TURNS_IN_DAY * days;
        restore_buffs(pc, |prop| matches!(prop, BuffProp::Rest));
        let fatigue = remove_fatigue(pc, days as usize);
        Toast::show("rest", &format!("{hp_gain} and {fatigue}"));
    })
}

fn remove_fatigue(pc: &mut PC, num: usize) -> String {
    const NAME: &str = simple::meta::FATIGUE.name;
    let fatigue: Vec<usize> = pc
        .inventory
        .iter()
        .filter_map(|(id, item)| match item.name.as_str() {
            NAME => Some(id),
            _ => None,
        })
        .take(num)
        .collect();
    if fatigue.is_empty() {
        "no fatigue found".to_string()
    } else {
        let len = fatigue.len();
        for id in fatigue {
            pc.inventory.remove(id);
        }
        format!("-{len} fatigue")
    }
}

fn restore_buffs<F>(pc: &mut PC, filter: F)
where
    F: FnMut(&BuffProp) -> bool + Copy,
{
    let buff_arr = pc
        .buffs
        .values_mut()
        .filter(|buff| buff.props.iter().any(filter));
    for buff in buff_arr {
        let count = buff.props.iter_mut().find_map(|prop| match prop {
            BuffProp::Count(count) => Some(count),
            _ => None,
        });
        if let Some(count) = count {
            count.curr = count.max
        }
    }
}

pub(super) fn updater() {
    on_buff_len_change();
    max_inventory_capacity();
    inventory_weights();
    encumberance();
    remove_expired_buffs();
    on_revealer();
}

fn max_inventory_capacity() {
    use crate::pc::attr::MAX_INVENTORY as MAX;

    let max_inv = PC::slice(|pc| {
        pc.followers
            .values()
            .map(|x| x.inv_incr())
            .fold(MAX, |acc, e| acc + e)
    });

    create_effect(move |_| {
        log!("↺  Max inventory capacity");
        PCSession::update(|x| x.max_inv = max_inv.get());
    });
}

/// Every time the length of the pc.inventory changes, the slots
/// each item occupies is also updated.
fn inventory_weights() {
    let on_change = create_memo(move |_| {
        PCSession::with_pc(|sesh, pc| {
            let len = pc.inventory.len();
            let max = sesh.max_inv;
            (len, max)
        })
    });

    create_effect(move |_| {
        log!("↺  Inventory weights and encumberance");
        let (_, max) = on_change.get();
        let mut is_encumbered = false;
        let weights: IndexMap<SlotRange> = PC::untracked(|pc| {
            let mut last = 1;
            pc.inventory.clone_map(|item| {
                let is_bulky = item.is_bulky();
                let result = if last > max {
                    is_encumbered = true;
                    SlotRange::Encumbered
                } else if is_bulky {
                    SlotRange::Double(last)
                } else {
                    SlotRange::Single(last)
                };
                last += is_bulky as usize + 1;
                result
            })
        });
        PCSession::update(|x| {
            x.inv_slots = weights;
            x.is_enumbered = is_encumbered;
        });
    });
}

fn encumberance() {
    use crate::buffs::conditions::ENCUMBERED;

    let is_encumbered = PCSession::slice(|sesh| sesh.is_enumbered);

    create_effect(move |_| {
        let debuff_pos = PC::untracked(|pc| pc.buffs.position(|buff| buff.name == ENCUMBERED.name));
        if is_encumbered.get() {
            if debuff_pos.is_none() {
                log!("↺  Adding encumbered debuff");
                PC::update(|pc| pc.buffs.add(ENCUMBERED.into()));
            }
        } else if let Some(i) = debuff_pos {
            log!("↺  Removing encumbered debuff");
            PC::update(|pc| pc.buffs.remove(i));
        }
    });
}

fn remove_expired_buffs() {
    let time = create_memo(move |_| PC::with(|pc| pc.turns.0));

    create_effect(move |_| {
        log!("↺  Checking for expired buffs");
        let time = time.get();
        PC::update(|pc| {
            let expired: Vec<usize> = pc
                .buffs
                .iter()
                // Filter expired buffs.
                .filter_map(|(id, buff)| {
                    if let Some(turns) = buff.find_expiry() {
                        if turns.is_expired(time) {
                            return Some(id);
                        }
                    }
                    None
                })
                .collect();
            for id in expired {
                pc.buffs.remove(id);
            }
        })
    });
}

// -----------------------------------
// BUFF LEN
// -----------------------------------

/// Run a battery of functions when the length of buffs change.
fn on_buff_len_change() {
    let buff_len = PC::slice(|pc| pc.buffs.len());

    create_effect(move |_| {
        let _ = buff_len.get();
        log!("↺  Buffs changed");
        log!("    1. Searching for stat changes");
        session_stats();
        log!("    2. Adding expiry to relevant buffs");
        PC::update(add_buff_expiry);
    });
}

fn session_stats() {
    let (mut base, overrides) = PC::untracked(|pc| {
        let overrides: Vec<_> = pc
            .buffs
            .values()
            .flat_map(|buff| {
                buff.props.iter().find_map(|prop| match prop {
                    BuffProp::StatOverride(stat, by) => Some((*stat, *by)),
                    _ => None,
                })
            })
            .collect();
        (pc.base_stats, overrides)
    });
    // Override the base with these values.
    for (stat, by) in overrides {
        *base.get_mut(stat) = by
    }
    PCSession::update(|sesh| sesh.stats = base)
}

/// Find buffs with the `BuffProp::Duration` values and add an `BuffProp::Expiry`.
fn add_buff_expiry(pc: &mut PC) {
    let time = pc.turns;
    for buff in pc.buffs.values_mut() {
        // Check if buff has a duration.
        let find_duration = buff.props.iter().find_map(|props| match props {
            BuffProp::Duration(x) => Some(x),
            _ => None,
        });
        if let Some(turns) = find_duration {
            // Add any expiry if it's missing one.
            if buff.find_expiry().is_none() {
                let mut expiry = *turns;
                expiry.add(&time);
                buff.props.push(crate::buffs::BuffProp::Expiry(expiry))
            }
        }
    }
}

fn on_revealer() {
    let target = web_sys::window().unwrap();
    let is_shown = create_memo(move |_| Revealer::is_shown());

    create_effect(move |_| {
        if is_shown.get() {
            log!("↺  Attaching scroll listener");
            let listener = EventListener::new(&target, "scroll", move |_| Revealer::hide());
            PCSession::update(|pc| pc.revealer_listen = Some(listener))
        } else {
            log!("↺  Removing scroll listener");
            PCSession::update(|pc| pc.revealer_listen = None)
        }
    });
}
