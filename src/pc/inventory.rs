use leptos::*;
use leptos_router::*;

mod backpack;
mod equipment;
mod funds;
mod search;
mod vault;

use backpack::*;
pub use equipment::*;
pub use vault::*;

use crate::pc::inventory::funds::EditableFunds;
use crate::pc::inventory::search::Search;

#[component]
pub fn Inventory(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class= "px-4">
            <h4 class= "border-b-2 border-purple-900 mb-4 text-center"> "Equipment" </h4>
            <Equipment />
            <h4 class= "border-b-2 border-sky-900 text-center"> "Backpack" </h4>
            <EditableFunds />
            <Backpack />
            <Search />
            <div class= "psuedo h-6" />
        </div>
    }
}

#[component]
pub fn InvNavbar(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Outlet />
    }
}
