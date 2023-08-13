use leptos::*;

mod backpack;
mod funds;
mod quick_access;
mod search;
mod stack_btn;
mod vault;

use self::backpack::Backpack;
use self::funds::EditableFunds;
use self::quick_access::QuickAccess;
use self::search::Search;
use crate::views::modal::ModalState;

#[component]
pub fn Inventory(cx: Scope) -> impl IntoView {
    let limit_scroll = move || {
        if ModalState::get(cx).is_some() {
            "px-4 overflow-y-hidden h-[85vh]"
        } else {
            "px-4"
        }
    };

    view! {
        cx,
        <div class=limit_scroll>
            <h5 class= "border-b-2 border-purple-900 mb-4 text-center"> "QUICK ACCESS" </h5>
            <QuickAccess />
            <h5 class= "border-b-2 border-sky-900 text-center mt-6"> "BACKPACK" </h5>
            <EditableFunds />
            <Backpack />
            <Search />
            <div class= "psuedo h-6" />
        </div>
    }
}
