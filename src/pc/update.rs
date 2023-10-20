use std::cmp;

use gloo::events::EventListener;
use leptos::logging::log;
use leptos::*;

use self::buffs::on_buff_change;
use super::session::Session;
use super::PC;
use crate::buffs::BuffProp;
use crate::items::meta::FATIGUE;
use crate::pc::realm::FollowerStat;
use crate::pc::session::SlotRange;
use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;
use crate::utils::turns::TURNS_IN_DAY;
use crate::utils::RwSignalEnhance;
use crate::views::revealer::Revealer;
use crate::views::toast::Toast;

mod buffs;

pub(super) fn on_rally() {
    PC::expect().update(|pc| {
        add_fatigue(pc);
        restore_buffs(pc, |prop| matches!(prop, BuffProp::Rally));
    })
}

pub(super) fn on_rest(days: u64, is_safe: bool) {
    PC::expect().update(|pc| {
        // Restore health.
        let health = cmp::min(days as i32, pc.health_dmg) * (is_safe as i32);
        pc.health_dmg -= health;
        pc.guard_dmg = 0;
        // Change days.
        pc.turns.0 += TURNS_IN_DAY * days;
        // Remove fatigue.
        let fatigue = remove_fatigue(pc, days as usize);
        Toast::show("rest", &format!("+{health} health and -{fatigue} fatigue"));

        // Restore buffs.
        restore_buffs(pc, |prop| matches!(prop, BuffProp::Rest));
    })
}
pub(super) fn updater() {
    on_buff_change();
    max_inventory_capacity();
    inventory_weights();
    encumberance();
    remove_expired_buffs();
    on_revealer();
    on_level();
    on_stats_changed();
}

fn add_fatigue(pc: &mut PC) {
    Toast::show("rally", "fatigue has been added to inventory");
    pc.inventory.add(FATIGUE.into());
}

fn remove_fatigue(pc: &mut PC, num: usize) -> usize {
    const NAME: &str = FATIGUE.name;
    let fatigue: Vec<usize> = pc
        .inventory
        .iter()
        .filter(|(_, item)| item.name == NAME)
        .map(|(id, _)| id)
        .take(num)
        .collect();
    for id in fatigue.iter() {
        pc.inventory.remove(*id);
    }
    fatigue.len()
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

fn on_stats_changed() {
    let stats = PC::slice(|pc| pc.base_stats);

    create_effect(move |_| {
        let _ = stats.get();
        log!("↺  Stats changed");
        buffs::session_stats();
    });
}

fn on_level() {
    let level = PC::slice(|pc| pc.class.1.level());

    // High priority level effects. Specifically for pages that require
    // this to be up-to-date before they can show their info.
    create_render_effect(move |_| {
        log!("↺  Level changed (render effects)");
        let _ = level.get();
        caster_status();
    });
    create_effect(move |_| {
        log!("↺  Level changed");
        let _ = level.get();
        health_increase();
    });
}

fn health_increase() {
    use crate::pc::attr::{GUARD, HEALTH};
    use crate::pc::PCStat;

    log!("    • Recalculating base health and guard");
    PC::expect().update(|pc| {
        let level = pc.class.1.level().get() as i32;
        let guard_bonus = pc.class.0.guard_bonus;
        let health = HEALTH + level - 1;
        let guard = GUARD + ((level - 1) * guard_bonus);
        *pc.base_stats.get_mut(PCStat::Health) = health;
        *pc.base_stats.get_mut(PCStat::Guard) = guard;
    })
}

fn caster_status() {
    log!("    • Checking caster status");
    let (arcane, divine) = PC::expect().with_untracked(|pc| {
        let (mut arcane, mut divine) = (0, 0);
        for buff in pc.buffs.values() {
            if let Some(second_word) = buff.name.split(' ').nth(1) {
                if second_word == "arcane" {
                    arcane += 1;
                } else if second_word == "divine" {
                    divine += 1;
                }
            }
        }
        (arcane, divine)
    });
    Session::expect().update(|sesh| {
        sesh.cast_divine = divine;
        sesh.cast_arcane = arcane;
    })
}

fn max_inventory_capacity() {
    use crate::pc::attr::INV_CAPACITY;

    let sesh = Session::expect();
    let max_inv = PC::slice(|pc| {
        pc.followers
            .values()
            .map(|follower| follower.stats.get(FollowerStat::Inventory))
            .fold(INV_CAPACITY, |acc, e| acc + e as usize)
    });

    create_effect(move |_| {
        log!("↺  Max inventory capacity");
        sesh.update(|x| x.max_inv = max_inv.get());
    });
}

/// Every time the length of the pc.inventory changes, the slots
/// each item occupies is also updated.
fn inventory_weights() {
    let (pc, sesh) = (PC::expect(), Session::expect());
    let on_change = create_memo(move |_| {
        let len = pc.with(|pc| pc.inventory.len());
        let max = sesh.with(|sesh| sesh.max_inv);
        (len, max)
    });

    create_effect(move |_| {
        log!("↺  Inventory changed");
        let (_, max) = on_change.get();
        let mut last = 0;
        let mut is_encumbered = false;
        let weights: IndexMap<SlotRange> = pc.with_untracked(|pc| {
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
        sesh.update(|sesh| {
            sesh.inv_slots = weights;
            sesh.is_encumbered = is_encumbered;
            sesh.empty_inv_slots = max.saturating_sub(last);
        })
    });
}

fn encumberance() {
    use crate::buffs::conditions::ENCUMBERED;
    let pc = PC::expect();
    let is_encumbered = Session::slice(|sesh| sesh.is_encumbered);

    create_effect(move |_| {
        let debuff_pos =
            pc.with_untracked(|pc| pc.buffs.position(|buff| buff.name == ENCUMBERED.name));
        if is_encumbered.get() {
            if debuff_pos.is_none() {
                log!("↺  Adding encumbered debuff");
                PC::expect().update(|pc| pc.buffs.add(ENCUMBERED.into()));
            }
        } else if let Some(i) = debuff_pos {
            log!("↺  Removing encumbered debuff");
            pc.update_discard(|pc| pc.buffs.remove(i));
        }
    });
}

fn remove_expired_buffs() {
    let time = create_memo(move |_| PC::expect().with(|pc| pc.turns.0));

    create_effect(move |_| {
        log!("↺  Checking for expired buffs");
        let time = time.get();
        PC::expect().update(|pc| {
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
    let is_shown = create_memo(move |_| Revealer::shown());
    let sesh = Session::expect();

    create_effect(move |_| {
        if is_shown.get() {
            log!("↺  Attaching scroll listener");
            let listener = EventListener::new(&target, "scroll", move |_| Revealer::hide());
            sesh.update(|pc| pc.revealer_listen = Some(listener))
        } else {
            log!("↺  Removing scroll listener");
            sesh.update(|pc| pc.revealer_listen = None)
        }
    });
}
