use leptos::logging::log;
use leptos::*;
use leptos_use::use_debounce_fn;

use crate::items::{Item, ItemProp};
use crate::pc::session::{Session, SlotRange};
use crate::pc::{AbiScores, Ability, PC};
use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;

pub fn reduce_by_fatigue() {
    let sesh = Session::expect();
    let fatigue = PC::slice(|pc| pc.fatigue);
    create_effect(move |_| {
        let fatigue = fatigue.get();
        sesh.update(|sesh| *sesh.isolated_scores.get_mut(Ability::MaxInventory) = -fatigue);
    });
}

pub fn on_item_change() {
    let (pc, sesh) = (PC::expect(), Session::expect());
    let inv_change = create_memo(move |_| {
        let len = pc.with(|pc| pc.inventory.len());
        let max = sesh.with(|sesh| sesh.abi_scores.get(Ability::MaxInventory));
        (len, max)
    });
    let update_inv_details = use_debounce_fn(
        move || {
            log!("> Inventory changed");
            let (_, max) = inv_change.get();
            abi_scores();
            item_sort();
            weights(max as usize);
        },
        100.0,
    );

    create_effect(move |_| {
        let _ = inv_change.get();
        update_inv_details()
    });
}

pub fn encumberance() {
    use crate::buffs::conditions::ENCUMBERED;
    let pc = PC::expect();
    let is_encumbered = Session::slice(|sesh| sesh.is_encumbered);

    create_effect(move |_| {
        let debuff_pos =
            pc.with_untracked(|pc| pc.buffs.position(|buff| buff.name == ENCUMBERED.name));
        if is_encumbered.get() {
            if debuff_pos.is_none() {
                log!("> Adding encumbered debuff");
                PC::expect().update(|pc| pc.buffs.add(ENCUMBERED.into()));
            }
        } else if let Some(i) = debuff_pos {
            log!("> Removing encumbered debuff");
            pc.update_discard(|pc| pc.buffs.remove(i));
        }
    });
}

fn item_sort() {
    log!("    | Sorting items by alphabet");
    let (pc, sesh) = (PC::expect(), Session::expect());
    let sorted = pc.with_untracked(|pc| {
        let mut items: Vec<_> = pc.inventory.iter().collect();
        items.sort_unstable_by(|a, b| a.1.name.cmp(&b.1.name));
        items.into_iter().map(|(id, _)| id).collect()
    });
    sesh.update(|sesh| sesh.sorted_inv = sorted);
}

/// Every time the length of the pc.inventory changes, the slots
/// each item occupies is also updated.
fn weights(max: usize) {
    log!("    | Calculating slot weight ranges");
    let (pc, sesh) = (PC::expect(), Session::expect());
    let mut last = 0;
    let mut is_encumbered = false;
    let item_ids = sesh.with_untracked(|sesh| sesh.sorted_inv.clone());
    let find_bulky = |item: &Item| {
        item.props
            .iter()
            .find_map(|prop| match prop {
                ItemProp::Bulky(x) => Some(*x),
                _ => None,
            })
            .unwrap_or(1)
    };
    let weights: Vec<_> = pc.with_untracked(|pc| {
        item_ids
            .into_iter()
            .map(|id| {
                let slots_used = pc.inventory.get(id).map(find_bulky).unwrap_or_default();
                last += slots_used;
                let range = if last > max {
                    is_encumbered = true;
                    SlotRange::Encumbered
                } else if slots_used > 1 {
                    SlotRange::Double(last - slots_used + 1, last)
                } else {
                    SlotRange::Single(last)
                };
                (id, range)
            })
            .collect()
    });
    sesh.update(|sesh| {
        sesh.inv_slots = IndexMap::from(weights);
        sesh.is_encumbered = is_encumbered;
        sesh.empty_inv_slots = max.saturating_sub(last);
    });
}

fn abi_scores() {
    log!("    | Finding any items with ability scores");
    let (pc, sesh) = (PC::expect(), Session::expect());
    let scores = pc.with_untracked(|pc| {
        let score_changes = pc.inventory.values().filter_map(|item| {
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
