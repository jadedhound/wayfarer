use leptos::*;

mod backpack;
mod funds;
mod quick_access;
mod search;
mod stack_btn;

use self::backpack::backpack;
use self::quick_access::quick_access;
use self::search::search_view;
use crate::pc::inventory::funds::editable_funds;
use crate::views::modal::ModalState;

pub fn inventory() -> impl IntoView {
    // TODO: Remove if no popup modal is used in the future.
    let limit_scroll = move || {
        ModalState::get()
            .map(|_| "overflow-y-hidden h-[85vh]")
            .unwrap_or_default()
    };

    view! {
        <div class=move || format!("flex flex-col gap-4 px-2 {}", limit_scroll())>
            <h5 class= "text-center"> "QUICK ACCESS" </h5>
            { quick_access() }
            <h5 class= "text-center"> "BACKPACK" </h5>
            { editable_funds() }
            { search_view() }
            { backpack() }
            <div class= "psuedo h-6" />
        </div>
    }
}
