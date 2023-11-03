use leptos::*;
use leptos_router::A;

use super::Shop;
use crate::pc::session::Session;
use crate::utils::rw_utils::RwUtils;

pub fn shop_list() -> impl IntoView {
    view! {
        <ShopButton shop=Shop::Adventurer class= "text-pink-400" />
        <ShopButton shop=Shop::Weaponsmith class= "text-pink-400" />
        <ShopButton shop=Shop::Alchemist class= "text-fuchsia-400" />
        <ShopButton shop=Shop::Armoursmith class= "text-fuchsia-400" />
        <ShopButton shop=Shop::Fletcher class= "text-purple-400" />
        <ShopButton shop=Shop::Illicit class= "text-purple-400" />
        <ShopButton shop=Shop::Arcane class= "text-green-500" />
        <ShopButton shop=Shop::Divine class= "text-green-500" />
    }
}

#[component]
fn ShopButton(shop: Shop, #[prop(optional)] class: &'static str) -> impl IntoView {
    if Session::expect().with(|sesh| shop.cannot_use(sesh)) {
        view! {
            <button class=format!("btn {class}") disabled=true>
                { shop.to_string() }
            </button>
        }
        .into_view()
    } else {
        view! {
            <A
                class=format!("btn bg-surface text-center {class}")
                href=format!("buy/{}", shop as usize)
            >
                { shop.to_string() }
            </A>
        }
        .into_view()
    }
}
