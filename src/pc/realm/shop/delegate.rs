use std::time::Duration;

use leptos::html::Div;
use leptos::*;
use leptos_use::use_debounce_fn;

use super::cart_view::cart_view;
use super::shop_view::shop_view;
use super::State;
use crate::icons;
use crate::items::Item;
use crate::utils::rw_utils::RwUtils;
use crate::views::modal::{ModalCenter, ModalLocation};
use crate::views::wealth::maybe_wealth;

pub fn shop_delegate() -> impl IntoView {
    State::provide();
    let is_cart = State::slice(|state| state.is_cart);

    let current_view = move || {
        if is_cart.get() {
            cart_view().into_view()
        } else {
            shop_view().into_view()
        }
    };

    view! {
        { current_view }
        { swap_fab }
        { modal_item_details }
    }
}

fn swap_fab() -> impl IntoView {
    let state = State::expect();
    let text = move || state.with(|state| if state.is_cart { "SHOP" } else { "CART" });
    let change_view = move |_| state.update(|state| state.is_cart = !state.is_cart);
    let item_num = State::slice(|state| state.cart.len());
    let div_ref = create_node_ref::<Div>();
    let notify_animation = use_debounce_fn(
        move || {
            spawn_local(async move {
                let div_ref = div_ref.get_untracked().unwrap();
                div_ref
                    .class_list()
                    .add_1("animate-notify")
                    .unwrap_or_default();
                gloo::timers::future::sleep(Duration::from_secs(1)).await;
                div_ref
                    .class_list()
                    .remove_1("animate-notify")
                    .unwrap_or_default();
            });
        },
        500.0,
    );

    create_effect(move |_| {
        let _ = item_num.get();
        let is_shop = state.with_untracked(|state| !state.is_cart);
        if is_shop && div_ref.get_untracked().is_some() {
            notify_animation();
        }
    });

    view! {
        <button
            class= "fixed bottom-4 right-4 btn bg-orange-800 !rounded-full flex-center gap-2 w-32 h-12"
            on:click=change_view
        >
            <div node_ref=div_ref> { item_num } </div>
            <div class= "psuedo w-[2px] h-full bg-orange-500" />
            <span> { text } </span>
        </button>
    }
}

fn modal_item_details() -> impl IntoView {
    let item = State::slice(|state| Item::from(*state.item_details));
    let item_view = move || item.get().into_view();
    let price = move || maybe_wealth(item.get().price());
    let count = move || item.get().find_counter().map(|count| count.max);

    view! {
        <ModalCenter location=ModalLocation::ShopItemDetails>
            { item_view }
            <div class= "flex items-center justify-between flex-wrap">
                { price }
                <div class="px-4 h-10 flex items-center gap-2" hidden=move || count().is_none()>
                    <div> { count } </div>
                    <div class= "w-4" inner_html=icons::STACK />
                </div>
            </div>
        </ModalCenter>
    }
}
