use leptos::*;
use leptos_router::A;

use super::Shop;

pub fn shop_list() -> impl IntoView {
    view! {
        { shop_button(Shop::Adventurer, "text-pink-400") }
        { shop_button(Shop::Weaponsmith, "text-pink-400") }
        { shop_button(Shop::Alchemist, "text-fuchsia-400") }
        { shop_button(Shop::Armoursmith, "text-fuchsia-400") }
        { shop_button(Shop::Fletcher, "text-purple-400") }
        { shop_button(Shop::Illicit, "text-purple-400") }
    }
}

fn shop_button(shop: Shop, class: &'static str) -> impl IntoView {
    view! {
        <A
            class=format!("btn bg-surface text-center {class}")
            href=format!("buy/{}", shop as usize)
        >
            { shop.to_string() }
        </A>
    }
}
