use leptos::*;
use leptos_router::*;

mod backpack;
mod equipment;
mod inv_item;
mod vault;

use backpack::*;
pub use equipment::*;
pub use vault::*;

#[component]
pub fn Inventory(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class= "px-4">
            <Equipment />
            <Backpack />
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
