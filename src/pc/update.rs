use leptos::*;

use crate::{
    pc::{pc_stat::StatArray, session::SlotRange},
    utils::{index_map::IndexMap, RwProvided},
};

use super::{pc_stat::PCStat, session::PCSession, PC};

pub(super) fn updater() {
    stats_by_buffs();
    inv_slots();
    remove_expired_buffs();
}

fn stats_by_buffs() {
    let on_buff_change = create_memo(move |_| PC::with(|pc| pc.buffs.len()));
    let on_base_change =
        create_memo(move |_| PC::with(|pc| pc.base_stats.iter().fold(0, |acc, e| acc + e.1)));

    create_effect(move |_| {
        log::info!("update: session stats");
        let _ = on_buff_change.get();
        let _ = on_base_change.get();
        let (base, buffs) = PC::untracked(|pc| {
            let base = pc.base_stats;
            let buffs: Vec<StatArray> = pc.buffs.values().filter_map(|x| x.stats).collect();
            (base, buffs)
        });
        PCSession::update(|sesh| {
            sesh.stats = base;
            buffs
                .into_iter()
                .for_each(|stat_arr| sesh.stats.merge(stat_arr))
        })
    });
}

/// Every time the length of the pc.inventory changes, the slots
/// each item occupies is also updated.
fn inv_slots() {
    let len = create_memo(move |_| {
        let a = PC::with(|pc| pc.inventory.len());
        let max = PCSession::with(|sesh| sesh.stats.get(PCStat::Inventory));
        (a, max)
    });

    create_effect(move |_| {
        log::info!("update: inventory slot weights");
        let (_, max) = len.get();
        let weights: IndexMap<SlotRange> = PC::untracked(|pc| {
            let mut last = 1;
            pc.inventory.clone_map(|item| {
                let result = if last > max as usize {
                    SlotRange::Encumbered
                } else if item.is_bulky {
                    SlotRange::Double(last)
                } else {
                    SlotRange::Single(last)
                };
                last += item.is_bulky as usize + 1;
                result
            })
        });
        PCSession::update(|x| x.inv_slots = weights);
    });
}

fn remove_expired_buffs() {
    let time = create_memo(move |_| PC::with(|pc| pc.turns.0));

    create_effect(move |_| {
        log::info!("update: checking for expired buffs");
        let time = time.get();
        PC::update(|pc| {
            let expired: Vec<usize> = pc
                .buffs
                .iter()
                .filter_map(|(id, buff)| {
                    if buff.duration.is_expired(time) {
                        Some(id)
                    } else {
                        None
                    }
                })
                .collect();
            for id in expired {
                pc.buffs.remove(id);
            }
        })
    });
}
