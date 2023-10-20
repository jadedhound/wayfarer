use leptos::*;
use leptos_router::A;

use super::followers::followers;
use crate::pc::realm::rest::Rest;
use crate::pc::realm::Shop;
use crate::pc::session::Session;
use crate::utils::rw_utils::RwUtils;

pub fn realm() -> impl IntoView {
    view! {
        <h4 class= "text-center"> "Followers" </h4>
        { followers }
        <h4 class= "text-center"> "Town" </h4>
        <div class= "grid grid-cols-2 gap-1">
            { shop_button(Shop::Alchemist) }
            { shop_button(Shop::Adventurer) }
            { shop_button(Shop::Blacksmith) }
            { shop_button(Shop::Arcane) }
            { shop_button(Shop::Divine) }
            <A href= "sell" class= "btn bg-surface col-span-2 text-orange-500 text-center py-2">
                "SELL ITEMS"
            </A>
        </div>
        <h4 class= "text-center"> "Rest" </h4>
        <Rest />
    }
}

fn shop_button(shop: Shop) -> impl IntoView {
    let text_colour = match shop {
        Shop::Arcane | Shop::Divine => "text-green-500",
        _ => "text-purple-400",
    };
    let new_row = matches!(shop, Shop::Arcane)
        .then_some("col-start-1")
        .unwrap_or_default();
    if Session::expect().with(|sesh| shop.cannot_use(sesh)) {
        view! {
            <button class=format!("btn py-2 {new_row}") disabled=true>
                { shop.to_string() }
            </button>
        }
        .into_view()
    } else {
        view! {
            <A
                class=format!("btn bg-surface text-center py-2 {text_colour} {new_row}")
                href=format!("buy/{}", shop as usize)
            >
                { shop.to_string() }
            </A>
        }
        .into_view()
    }
}
