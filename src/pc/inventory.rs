use leptos::*;

use self::backpack::backpack;
use self::search::search_view;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

mod backpack;
mod count_button;
mod recently_removed;
mod search;

pub fn inventory() -> impl IntoView {
    let spacer_hidden = PC::slice(|pc| pc.inventory.vacancy().is_some_and(|slots| slots > 0));

    view! {
        { search_view }
        { backpack }
        { recently_removed::button }
        { recently_removed::modal }
        <div class= "psuedo h-16" hidden=spacer_hidden />
    }
}
