use leptos::*;
use strum::EnumCount;

use super::{PCStat, PC};
use crate::items::buffs::{Buff, FeatOrStat};
use crate::items::features::Feature;
use crate::items::Item;
use crate::utils::rw_context;

#[derive(Clone)]
pub(super) struct PCSession {
    pub stats: [i32; PCStat::COUNT],
    pub features: Vec<Feature>,
    pub rcp_index: usize,
}

impl PCSession {
    /// Calculates modified stats from the PC's state.
    /// Relies on `PC` to have been provided already.
    pub fn provide(cx: Scope) {
        let pc = rw_context::<PC>(cx);
        // Create base session
        let sesh = pc.with_untracked(|pc| Self {
            stats: pc.base_stats,
            features: Vec::new(),
            rcp_index: 0,
        });
        let sesh = create_rw_signal(cx, sesh);
        provide_context(cx, sesh);

        pc.with_untracked(|pc| {
            pc.equipment
                .iter()
                .flatten()
                .for_each(|x| add_equipment(cx, x));
            pc.conditions.iter().for_each(|x| add_buff(cx, x));
        });
    }
}

/// Add a given item to stats and features calculated already.
pub fn add_equipment(cx: Scope, item: &Item) {
    rw_context::<PCSession>(cx).update(|sesh| {
        if let Some(stats) = item.spec.as_stat_arr() {
            for (i, ele) in stats.0.iter().enumerate() {
                sesh.stats[i] += ele;
            }
        }
        if let Some(feat) = item.spec.as_feat() {
            sesh.features.push(feat.clone());
        }
    })
}

/// Removes a given item from stats and features.
pub fn rm_equipment(cx: Scope, item: &Item) {
    rw_context::<PCSession>(cx).update(|sesh| {
        if let Some(stats) = item.spec.as_stat_arr() {
            for (i, ele) in stats.0.iter().enumerate() {
                sesh.stats[i] -= ele;
            }
        }
        if let Some(feat) = item.spec.as_feat() {
            let i = sesh.features.iter().position(|x| x.name == feat.name);
            if let Some(i) = i {
                sesh.features.remove(i);
            }
        }
    })
}

pub fn add_buff(cx: Scope, buff: &Buff) {
    rw_context::<PCSession>(cx).update(|sesh| match &buff.effect {
        FeatOrStat::Feat(x) => sesh.features.push(x.clone()),
        FeatOrStat::Stat(x) => x.0.iter().enumerate().for_each(|(i, x)| sesh.stats[i] += x),
    })
}

pub fn rm_buff(cx: Scope, buff: &Buff) {
    rw_context::<PCSession>(cx).update(|sesh| match &buff.effect {
        FeatOrStat::Feat(x) => {
            let i = sesh.features.iter().position(|y| x.name == y.name);
            if let Some(i) = i {
                sesh.features.remove(i);
            }
        }
        FeatOrStat::Stat(x) => x.0.iter().enumerate().for_each(|(i, x)| sesh.stats[i] -= x),
    })
}
