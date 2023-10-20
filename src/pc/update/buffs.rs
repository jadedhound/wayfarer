use leptos::logging::log;
use leptos::*;

use super::PC;
use crate::buffs::BuffProp;
use crate::pc::session::Session;
use crate::utils::rw_utils::RwUtils;

/// Run a battery of functions when the length of buffs change.
pub(super) fn on_buff_change() {
    let pc = PC::expect();
    let buff_len = PC::slice(|pc| pc.buffs.len());

    create_effect(move |_| {
        let _ = buff_len.get();
        log!("↺  Buffs changed");
        session_stats();
        log!("    2. Adding expiry to relevant buffs");
        pc.update(add_buff_expiry);
    });
}

pub(super) fn session_stats() {
    log!("    1. Searching for stat changes");
    let (mut base, overrides) = PC::expect().with_untracked(|pc| {
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
    Session::expect().update(|sesh| sesh.stats = base)
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
                expiry.add(time);
                buff.props.push(crate::buffs::BuffProp::Expiry(expiry))
            }
        }
    }
}
