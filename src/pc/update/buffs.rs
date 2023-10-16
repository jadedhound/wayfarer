use leptos::logging::log;
use leptos::*;

use super::PC;
use crate::buffs::BuffProp;
use crate::pc::session::PCSession;
use crate::utils::RwProvided;

/// Run a battery of functions when the length of buffs change.
pub(super) fn on_buff_change() {
    let buff_len = PC::slice(|pc| pc.buffs.len());

    create_effect(move |_| {
        let _ = buff_len.get();
        log!("↺  Buffs changed");
        log!("    1. Searching for stat changes");
        session_stats();
        log!("    2. Adding expiry to relevant buffs");
        PC::update(add_buff_expiry);
        log!("    3. Checking caster status");
        caster_status();
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

fn caster_status() {
    let (arcane, divine) = PC::with(|pc| {
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
    PCSession::update(|sesh| {
        sesh.cast_divine = divine;
        sesh.cast_arcane = arcane;
    })
}
