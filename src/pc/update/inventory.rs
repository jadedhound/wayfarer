use leptos::logging::{log, warn};
use leptos::*;
use leptos_use::use_debounce_fn;

use crate::items::{Item, ItemProp};
use crate::pc::session::Session;
use crate::pc::{AbiScores, Ability, PC};
use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;

pub fn on_item_change() {
    let (pc, sesh) = (PC::expect(), Session::expect());
    let inv_change = create_memo(move |_| pc.with(|pc| pc.backpack.len()));
    let update_inv_details = use_debounce_fn(
        move || {
            log!("> Inventory changed");
            let _ = inv_change.get();
            abi_scores();
        },
        100.0,
    );

    create_effect(move |_| {
        let _ = inv_change.get();
        update_inv_details()
    });
}

pub fn encumberance() {
    let pc = PC::expect();
    let is_encumbered = PC::slice(|pc| pc.backpack.vacancy().is_none());

    create_effect(move |_| warn!("is_encumbered is not implemented"));
}

fn abi_scores() {
    log!("    | Finding any items with ability scores");
    let (pc, sesh) = (PC::expect(), Session::expect());
    let scores = pc.with_untracked(|pc| {
        let score_changes = pc.backpack.values().filter_map(|item| {
            item.props.iter().find_map(|x| match x {
                ItemProp::Score(x, y) => Some((*x, *y)),
                _ => None,
            })
        });
        let mut scores = AbiScores::default();
        for (abi, score) in score_changes {
            *scores.get_mut(abi) += score
        }
        scores
    });
    sesh.update(|sesh| sesh.inv_scores = scores);
}
