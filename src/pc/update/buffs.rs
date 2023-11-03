use std::cmp;

use leptos::logging::log;
use leptos::*;
use leptos_use::use_debounce_fn;

use super::PC;
use crate::buffs::BuffProp;
use crate::pc::session::Session;
use crate::pc::AbiScores;
use crate::utils::rw_utils::RwUtils;

/// Run a battery of functions when the length of buffs change.
pub fn on_change() {
    let pc = PC::expect();
    let buff_len = PC::slice(|pc| pc.buffs.len());
    let buff_len = leptos_use::signal_debounced(buff_len, 500.0);

    create_effect(move |_| {
        let _ = buff_len.get();
        log!("> Buffs changed");
        scores();
        score_overrides();
        pc.update(add_buff_expiry);
    });
}

pub fn remove_expired() {
    let time = create_memo(move |_| PC::expect().with(|pc| pc.turns.0));
    let check_for_expired = use_debounce_fn(
        move || {
            log!("> Checking for expired buffs");
            let time = time.get();
            PC::expect().update(|pc| {
                let expired: Vec<usize> = pc
                    .buffs
                    .iter()
                    // Compare found expiry with time and if expired return `Some(id)`.
                    .filter_map(|(id, buff)| {
                        buff.find_expiry()
                            .and_then(|turns| turns.is_expired(time).then_some(id))
                    })
                    .collect();
                for id in expired {
                    pc.buffs.remove(id);
                }
            })
        },
        500.0,
    );

    create_effect(move |_| {
        let _ = time.get();
        check_for_expired();
    });
}

fn scores() {
    log!("    | Searching for score increases");
    let buff_scores = PC::expect().with_untracked(|pc| {
        let score_changes = pc
            .buffs
            .values()
            .flat_map(|buff| {
                buff.props.iter().find_map(|prop| match prop {
                    BuffProp::Score(stat, by) => Some((*stat, *by)),
                    _ => None,
                })
            })
            .collect::<Vec<_>>();
        let mut scores = AbiScores::default();
        for (abi, score) in score_changes {
            if score < 0 {
                // Always apply debuffs.
                *scores.get_mut(abi) += score
            } else {
                // Only apply the highest buff.
                let curr = scores.get_mut(abi);
                *curr = cmp::max(*curr, score);
            }
        }
        scores
    });
    Session::expect().update(|sesh| sesh.buff_scores = buff_scores)
}

fn score_overrides() {
    log!("    | Searching for score overrides");
    let overrides = PC::expect().with_untracked(|pc| {
        pc.buffs
            .values()
            .flat_map(|buff| {
                buff.props.iter().find_map(|prop| match prop {
                    BuffProp::ScoreOverride(stat, by) => Some((*stat, *by)),
                    _ => None,
                })
            })
            .collect::<Vec<_>>()
    });
    Session::expect().update(|sesh| sesh.override_scores = overrides)
}

/// Find buffs with the `BuffProp::Duration` values and add an `BuffProp::Expiry`.
fn add_buff_expiry(pc: &mut PC) {
    log!("    | Adding expiry to relevant buffs");
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
