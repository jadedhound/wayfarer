use std::cmp;

use gloo::events::EventListener;
use leptos::logging::log;
use leptos::*;

use self::buffs::on_buff_change;
use super::session::PCSession;
use super::PC;
use crate::buffs::BuffProp;
use crate::items::FATIGUE;
use crate::pc::realm::FolStat;
use crate::pc::session::SlotRange;
use crate::utils::index_map::IndexMap;
use crate::utils::turns::TURNS_IN_DAY;
use crate::utils::RwProvided;
use crate::views::revealer::Revealer;
use crate::views::toast::Toast;

mod buffs;

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
    const NAME: &str = FATIGUE.name;
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
    on_buff_change();
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
            .map(|follower| follower.stats.get(FolStat::Mule))
            .fold(MAX, |acc, e| acc + e as usize)
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
        log!("↺  Inventory changed");
        let (_, max) = on_change.get();
        let mut last = 0;
        let mut is_encumbered = false;
        let weights: IndexMap<SlotRange> = PC::untracked(|pc| {
            pc.inventory.clone_map(|item| {
                let is_bulky = item.is_bulky();
                last += is_bulky as usize + 1;
                if last > max {
                    is_encumbered = true;
                    SlotRange::Encumbered
                } else if is_bulky {
                    SlotRange::Double(last)
                } else {
                    SlotRange::Single(last)
                }
            })
        });
        PCSession::update(|x| {
            x.inv_slots = weights;
            x.is_encumbered = is_encumbered;
            x.empty_inv_slots = max.saturating_sub(last);
        });
    });
}

fn encumberance() {
    use crate::buffs::conditions::ENCUMBERED;

    let is_encumbered = PCSession::slice(|sesh| sesh.is_encumbered);

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
