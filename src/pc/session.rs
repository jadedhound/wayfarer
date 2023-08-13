use std::collections::HashMap;

use leptos::*;
use strum::EnumCount;

use super::{PCStat, PC};
use crate::items::buffs::Buff;
use crate::items::StatArr;
use crate::utils::rw_context;

#[derive(Clone)]
pub(super) struct PCSession {
    pub stats: [i32; PCStat::COUNT],
    pub rcp_index: usize,
    pub inv_slots: HashMap<usize, (usize, usize)>,
}

impl PCSession {
    /// Calculates modified stats from the PC's state.
    /// Relies on `PC` to have been provided already.
    pub fn provide(cx: Scope) {
        let pc = rw_context::<PC>(cx);
        // Create base session
        let sesh = pc.with_untracked(|pc| Self {
            stats: pc.base_stats,
            rcp_index: 0,
            inv_slots: HashMap::new(),
        });
        let sesh = create_rw_signal(cx, sesh);
        provide_context(cx, sesh);

        update_inv_slots(cx);
        pc.with_untracked(|pc| {
            pc.buffs
                .values()
                .for_each(|x| sesh.update(|sesh| sesh.add_buff(x)));
        });
    }

    fn modify_stats(&mut self, arr: StatArr, modify: i32) {
        for (stat, ele) in arr.iter_with_stat() {
            self.stats[stat.index()] += ele * modify;
        }
    }

    /// Adds stats provided by buffs.
    pub fn add_buff(&mut self, buff: &Buff) {
        if let Some(stats) = buff.stats {
            self.modify_stats(stats, 1);
        }
    }

    /// Remvoes stats provided by buffs.
    pub fn rm_buff(&mut self, buff: &Buff) {
        if let Some(stats) = buff.stats {
            self.modify_stats(stats, -1);
        }
    }
}

/// Every time the length of the pc.inventory changes, the slots
/// each item occupies is also updated.
fn update_inv_slots(cx: Scope) {
    let pc = rw_context::<PC>(cx);
    let len = create_memo(cx, move |_| pc.with(|pc| pc.inventory.len()));
    create_effect(cx, move |_| {
        let _ = len.get();
        let weights: HashMap<usize, (usize, usize)> = pc.with(|pc| {
            let mut last = 0;
            pc.inventory
                .iter()
                .map(|(id, item)| {
                    let result = (*id, (last + 1, last + item.weight as usize));
                    last = result.1 .1;
                    result
                })
                .collect()
        });
        rw_context::<PCSession>(cx).update(|sesh| {
            sesh.inv_slots = weights;
        })
    })
}
