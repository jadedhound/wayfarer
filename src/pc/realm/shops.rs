use leptos::*;
use leptos_router::NavigateOptions;
use strum::IntoEnumIterator;

use crate::pc::session::PCSession;
use crate::pc::shops::Shop;
use crate::utils::RwProvided;

pub(super) fn shops() -> impl IntoView {
    let shop_list = Shop::iter().map(shop_btn).collect_view();

    view! {
        <div class= "grid grid-cols-2 gap-1">
            { shop_list }
        </div>
    }
}

fn shop_btn(shop: Shop) -> impl IntoView {
    let on_click = move || {
        PCSession::update(|sesh| sesh.active_shop = shop);
        let pc_id = PCSession::with(|sesh| sesh.pc_id);
        let navigate = leptos_router::use_navigate();
        navigate(&format!("/pc/{pc_id}/shop"), NavigateOptions::default());
    };

    view! {
        <button
            class= "btn bg-surface text-center py-2"
            on:click=move |_| on_click()
        >
            { shop.to_string() }
        </button>
    }
}
