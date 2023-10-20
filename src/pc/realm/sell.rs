use leptos::*;

use crate::icons;
use crate::items::meta::FATIGUE;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;
use crate::views::funds::{maybe_funds, short_funds};
use crate::views::modal::{ModalCenter, ModalState};
use crate::views::revealer::Revealer;

#[derive(Default)]
struct State {
    // A vector of item IDs.
    cart: Vec<usize>,
    modal_item_id: usize,
}

impl RwUtils for State {
    type Item = Self;
}

#[component]
pub fn Sell() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::provide());
    let cart_is_full = move || {
        let pc_len = pc.with(|pc| pc.inventory.len());
        let cart_len = state.with(|state| state.cart.len());
        cart_len >= pc_len
    };
    let cart_empty = State::slice(|state| state.cart.is_empty());

    view! {
        <h4 class= "text-center"> "Sell" </h4>
        <div class= "shaded-table" hidden=cart_empty>
            <Sales />
        </div>
        <div class= "italic text-center" hidden=move || !cart_empty.get()>
            "Cart is empty."
        </div>
        <h4 class= "text-center"> "Profit" </h4>
        <Profit />
        <h4 class= "text-center"> "Inventory" </h4>
        <div
            class= "shaded-table"
            hidden=cart_is_full
        >
            <Inventory />
        </div>
        <SellButton />
        <ItemDetails />
    }
}

#[component]
fn Sales() -> impl IntoView {
    let state = State::expect();
    let on_click = move |id: usize| {
        state.update(|state| {
            if let Some(id) = state.cart.iter().position(|&cart_id| id == cart_id) {
                state.cart.remove(id);
            }
        })
    };
    State::slice(move |state| {
        state
            .cart
            .iter()
            .map(|&id| {
                view! { <ItemView id on_click /> }
            })
            .collect_view()
    })
}

#[component]
fn ItemView<F>(id: usize, on_click: F) -> impl IntoView
where
    F: Fn(usize) + 'static,
{
    let (pc, state) = (PC::expect(), State::expect());
    let item = pc.with_untracked(|pc| pc.inventory.expect(id));
    let details_hidden = item.props.is_empty();
    let price = maybe_funds(item.price());
    let open_details = move |_| {
        ModalState::show(10);
        state.update(|x| x.modal_item_id = id);
    };

    view! {
        <div class= "p-2 flex gap-2">
            <button
                class= "w-4 stroke-sky-500"
                on:click=open_details
                hidden=details_hidden
                inner_html=icons::INFO
            />
            <button
                class= "capitalise w-12 grow text-left"
                on:click= move |_| on_click(id)
            >
                { &item.name }
            </button>
            { price }
        </div>
    }
}

#[component]
fn Profit() -> impl IntoView {
    let pc = PC::expect();
    let tallied_total = State::slice(move |state| pc.with(|pc| calculate_profit(state, pc)));
    let profit = move || tallied_total.get() * 8 / 10;
    let curr = PC::slice(move |pc| pc.wealth + profit());

    view! {
        <div class= "grid grid-cols-2 gap-x-2">
            <div class= "font-tight text-right"> "TALLIED COST:" </div>
            { move || short_funds(tallied_total.get()) }
            <div class= "font-tight text-right"> "MERCHANT FEE:" </div>
            <div class= "font-tight text-red-500"> "-20%" </div>
            <div class= "font-tight text-right"> "PROFIT:" </div>
            { move || short_funds(profit()) }
            <div class= "font-tight text-right"> "FINAL FUNDS:" </div>
            { move || short_funds(curr.get()) }
        </div>
    }
}

fn calculate_profit(state: &State, pc: &PC) -> u32 {
    state
        .cart
        .iter()
        .flat_map(|id| pc.inventory.get(*id))
        .map(|item| item.price())
        .sum::<u32>()
}

#[component]
fn Inventory() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let on_click = move |id: usize| state.update_discard(|state| state.cart.push(id));
    State::slice(move |state| {
        let mut item_ids: Vec<_> = pc.with(|pc| {
            pc.inventory
                .iter()
                .filter(|&(_, item)| item.name.as_str() != FATIGUE.name)
                .map(|(id, _)| id)
                .collect()
        });
        for id in state.cart.iter() {
            if let Some(id) = item_ids.iter().position(|item_id| id == item_id) {
                item_ids.remove(id);
            }
        }
        item_ids
            .into_iter()
            .map(|id| {
                view! { <ItemView id on_click /> }
            })
            .collect_view()
    })
}

#[component]
fn SellButton() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let no_cart_items = State::slice(|state| state.cart.is_empty());
    let sell_items = move |_| {
        Revealer::hide();
        let price = state.with(|state| pc.with(|pc| calculate_profit(state, pc) * 8 / 10));
        let cart_ids = state.with(|state| state.cart.clone());
        state.update_discard(|state| state.cart.clear());
        pc.update(|pc| {
            // Remove items.
            cart_ids.into_iter().for_each(|id| {
                pc.inventory.remove(id);
            });
            // Add price.
            pc.wealth += price;
        })
    };
    let confirm_prompt = move |_| Revealer::show('s', 0);

    view! {
        <div class= "relative">
            <button
                class= "btn bg-surface py-2 relative w-full"
                on:click=confirm_prompt
                disabled=no_cart_items
            >
                "SELL"
            </button>
            <button
                class= "btn bg-green-800 py-2 absolute h-full w-full top-0 left-0 z-40"
                hidden=move || !Revealer::is_shown('s', 0)
                on:click=sell_items
            >
                "CONFIRM"
            </button>
        </div>
    }
}

#[component]
fn ItemDetails() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let item = move || {
        let id = state.with(|state| state.modal_item_id);
        pc.with(|pc| pc.inventory.expect(id))
    };
    let stacks = move || {
        item()
            .find_counter()
            .map(|count| format!("Stack of {}.", count.max))
    };
    view! {
        <ModalCenter id=10>
            { move || item().into_view() }
            { stacks }
        </ModalCenter>
    }
}
