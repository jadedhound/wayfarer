use leptos::*;

use super::State;
use crate::items::{Item, ItemRef};
use crate::pc::realm::shop::shop_item_view;
use crate::pc::session::Session;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;
use crate::views::revealer::{RevLocation, Revealer};
use crate::views::wealth::wealth_short;

pub fn cart_view() -> impl IntoView {
    let state = State::expect();
    let to_view = move |(id, item_ref): (usize, &'static ItemRef)| {
        let weight = item_ref.find_bulky().unwrap_or(1);
        let remove_from_cart = move || {
            state.update(|state| {
                state.weight = state.weight.saturating_sub(weight);
                state.price = state.price.saturating_sub(item_ref.price());
                state.cart.remove(id);
            })
        };
        shop_item_view(item_ref, remove_from_cart)
    };
    let cart = move || {
        state.with(|state| {
            state
                .cart
                .iter()
                .map(|(id, item_ref)| (id, *item_ref))
                .collect::<Vec<_>>()
        })
    };

    view! {
        <h3 class= "text-center"> { "Cart" } </h3>
        <div
            class= "flex flex-col shaded-table"
            hidden=move || state.with(|state| state.cart.is_empty())
        >
            <For
                each=cart
                key=|(id, _)| *id
                children=to_view
            />
        </div>
        { clear }
        { price }
        { purchase_button }
    }
}

fn clear() -> impl IntoView {
    let state = State::expect();
    let disabled = State::slice(|state| state.cart.is_empty());
    let clear = move |_| state.reset();
    view! {
        <button
            class= "btn bg-red-800"
            on:click=clear
            disabled=disabled
        >
            "CLEAR CART"
        </button>
    }
}

fn price() -> impl IntoView {
    let (pc, sesh, state) = (PC::expect(), Session::expect(), State::expect());
    let price = create_read_slice(state, |state| state.price);
    let funds_left = State::slice(move |state| {
        let curr = state.price;
        let max = pc.with(|pc| pc.wealth);
        (curr <= max).then_some(max.saturating_sub(curr))
    });
    let funds_left = move || {
        if let Some(funds_left) = funds_left.get() {
            wealth_short(funds_left).into_view()
        } else {
            view! {
                <div class= "font-tight text-red-500"> "Not enough funds" </div>
            }
            .into_view()
        }
    };
    let inv_left = create_memo(move |_| {
        let curr = state.with(|state| state.weight);
        let max = sesh.with(|sesh| sesh.empty_inv_slots);
        (curr <= max).then_some(max.saturating_sub(curr))
    });
    let inv_left = move || {
        if let Some(inv_left) = inv_left.get() {
            inv_left.into_view()
        } else {
            view! {
                <div class= "font-tight text-red-500"> "Encumbered" </div>
            }
            .into_view()
        }
    };

    view! {
        <div class= "grid grid-cols-2 items-center gap-x-2 border-y-2 border-yellow-500 py-2">
            <div class= "text-right font-tight"> "Price:" </div>
            { move || wealth_short(price.get()) }
            <div class= "text-right font-tight"> "Funds left:" </div>
            { funds_left }
            <div class= "text-right font-tight"> "Inventory left:" </div>
            { inv_left }
        </div>
    }
}

fn purchase_button() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let cannot_buy =
        State::slice(move |state| state.cart.is_empty() || pc.with(|pc| pc.wealth) < state.price);
    let buy_items = move |_| {
        let (items, price) = state.with(|state| (state.cart.clone(), state.price));
        state.reset();
        pc.update(|pc| {
            for item in items.values() {
                pc.inventory.add(Item::from(**item))
            }
            pc.wealth -= price;
        });
        Revealer::hide()
    };

    view! {
        <div class= "relative">
            <button
                class= "btn bg-surface w-full"
                on:click=move |_| Revealer::show(RevLocation::ShopBuy, 0)
                disabled=cannot_buy
            >
                "PURCHASE"
            </button>
            <div hidden=move || Revealer::is_hidden(RevLocation::ShopBuy, 0)>
                <button
                    class= "absolute top-0 btn bg-blue-800 h-full w-full z-40"
                    on:click=buy_items
                >
                    "CONFIRM"
                </button>
            </div>
        </div>
    }
}
