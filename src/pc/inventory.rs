use leptos::*;

use self::backpack::backpack;
use self::search::search_view;
use crate::pc::session::Session;
use crate::pc::Ability;
use crate::utils::rw_utils::RwUtils;

mod backpack;
mod count_button;
mod recently_removed;
mod search;
mod wealth;

pub fn inventory() -> impl IntoView {
    let spacer_hidden = Session::slice(|sesh| {
        let max = sesh.abi_scores.get(Ability::MaxInventory) as usize;
        let last_weight = sesh
            // Get the last item in the list AFTER sorting.
            .sorted_inv
            .iter()
            .last()
            .and_then(|&id| {
                // Find its weight.
                sesh.inv_slots
                    .get(id)
                    // Default to max for encumbered.
                    .map(|slot| slot.largest().unwrap_or(max))
            })
            // Default to 0 for every other case.
            .unwrap_or(0);
        last_weight < max
    });

    view! {
        <h4 class= "text-center"> "Wealth" </h4>
        { wealth::wealth }
        <h4 class= "text-center"> "Backpack" </h4>
        { search_view }
        { backpack }
        { recently_removed::button }
        { recently_removed::modal }
        <div class= "psuedo h-16" hidden=spacer_hidden />
    }
}
