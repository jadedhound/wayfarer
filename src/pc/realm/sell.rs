use leptos::*;
use strum::{Display, EnumCount, FromRepr};

use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;
use crate::views::confirm_button::ConfirmButton;
use crate::views::modal::{ModalCenter, ModalLocation, ModalState};
use crate::views::revealer::RevLocation;
use crate::views::wealth::{maybe_wealth, wealth_short};

#[derive(Default)]
struct State {
    // A vector of item IDs.
    cart: Vec<usize>,
    modal_item_id: usize,
    reputation: Reputation,
}

impl RwUtils for State {}

#[derive(Clone, Copy, Default, EnumCount, FromRepr, Display, PartialEq)]
enum Reputation {
    Disliked,
    #[default]
    Neutral,
    Liked,
    Honoured,
    Revered,
}

impl Reputation {
    fn percent(&self) -> u32 {
        match self {
            Reputation::Disliked => 60,
            Reputation::Neutral => 70,
            Reputation::Liked => 80,
            Reputation::Honoured => 90,
            Reputation::Revered => 100,
        }
    }
}

pub fn sell() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::provide());
    let cart_is_full = move || {
        let pc_len = pc.with(|pc| pc.backpack.len());
        let cart_len = state.with(|state| state.cart.len());
        cart_len >= pc_len
    };
    let cart_empty = State::slice(|state| state.cart.is_empty());

    view! {
        <h4 class= "text-center"> "Sell" </h4>
        <div class= "shaded-table" hidden=cart_empty>
            { sales }
        </div>
        <div class= "italic text-center" hidden=move || !cart_empty.get()>
            "Cart is empty."
        </div>
        <h4 class= "text-center"> "Profit" </h4>
        { profit }
        <h4 class= "text-center"> "Inventory" </h4>
        <div
            class= "shaded-table"
            hidden=cart_is_full
        >
            { inventory }
        </div>
        { sell_button }
        <ItemDetails />
    }
}

fn sales() -> impl IntoView {
    let state = State::expect();
    let on_click = move |id: usize| {
        state.update(|state| {
            if let Some(id) = state.cart.iter().position(|&cart_id| id == cart_id) {
                state.cart.remove(id);
            }
        })
    };
    let ids_to_sell = move || state.with(|state| state.cart.clone());

    view! {
        <For
            each=ids_to_sell
            key=|id| *id
            children=move |id| item_view(id, on_click)
        />
    }
}

fn inventory() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let on_click = move |id: usize| state.update_discard(|state| state.cart.push(id));
    let inventory = move || {
        let selling_items = state.with(|state| state.cart.clone());
        pc.with(|pc| {
            pc.backpack
                .iter()
                .filter(|(id, _)| !selling_items.contains(id))
                .map(|(id, _)| id)
                .collect::<Vec<_>>()
        })
    };

    view! {
        <For
            each=inventory
            key=|id| *id
            children=move |id| item_view(id, on_click)
        />
    }
}

fn item_view<F>(id: usize, on_click: F) -> impl IntoView
where
    F: Fn(usize) + 'static,
{
    let (pc, state) = (PC::expect(), State::expect());
    let item = pc.with_untracked(|pc| pc.backpack.get(id).cloned().unwrap());
    let details_hidden = item.props.is_empty();
    let price = maybe_wealth(item.price());
    let open_details = move |_| {
        ModalState::show(ModalLocation::ShopItemDetails);
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

fn reputation() -> impl IntoView {
    let state = State::expect();
    let curr_rep = State::slice(|state| state.reputation);
    let change_rep = move |by: isize| {
        let i = (curr_rep.get() as isize + by) as usize;
        let rep = Reputation::from_repr(i).unwrap_or_default();
        state.update(|state| state.reputation = rep);
    };
    let rep_text = move || {
        let curr_rep = curr_rep.get();
        format!("{curr_rep}: -{}%", 100 - curr_rep.percent())
    };
    let text_colour = move || match curr_rep.get() {
        Reputation::Disliked => "text-red-500",
        Reputation::Neutral => "text-red-200",
        Reputation::Liked => "",
        Reputation::Honoured => "text-green-200",
        Reputation::Revered => "text-green-500",
    };
    view! {
        <div class= "flex">
            <button
                class= "disabled:fill-zinc-500"
                on:click=move |_| change_rep(-1)
                disabled=move || { (curr_rep.get() as usize) < 1 }
            >
                <div class= "w-5 rotate-180" inner_html=icons::RIGHT_CHEV />
            </button>
            <div class=move || format!("w-12 grow text-center {}", text_colour())>
                { rep_text }
            </div>
            <button
                class= "disabled:fill-zinc-500"
                on:click=move |_| change_rep(1)
                disabled=move || { curr_rep.get() as usize >= Reputation::COUNT - 1 }
            >
                <div class= "w-5" inner_html=icons::RIGHT_CHEV />
            </button>
        </div>
    }
}

fn profit() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let value = create_memo(move |_| with!(|pc, state| calc_value(pc, state)));
    let profit = move || state.with(|state| calc_profit(value.get(), state));

    view! {
        <div class= "grid grid-cols-2 gap-x-2">
            <div class= "font-tight text-right"> "VALUE:" </div>
            { move || wealth_short(value.get()) }
            <div class= "font-tight text-right"> "REPUTATION:" </div>
            { reputation }
            <div class= "font-tight text-right"> "PROFIT:" </div>
            { move || wealth_short(profit()) }
        </div>
    }
}

fn calc_value(pc: &PC, state: &State) -> u32 {
    state
        .cart
        .iter()
        .flat_map(|id| pc.backpack.get(*id))
        .map(|item| item.price())
        .sum::<u32>()
}

fn calc_profit(value: u32, state: &State) -> u32 {
    value * state.reputation.percent() / 100
}

fn sell_button() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let no_cart_items = State::slice(|state| state.cart.is_empty());
    let sell_items = move || {
        let price = with!(|pc, state| calc_profit(calc_value(pc, state), state));
        let cart_ids = state.with(|state| state.cart.clone());
        state.update_discard(|state| state.cart.clear());
        pc.update(|pc| {
            // Remove items.
            for id in cart_ids {
                if let Some(item) = pc.backpack.remove(id) {
                    pc.recently_removed.push_unique(item)
                }
            }
            // Add price.
            pc.wealth += price;
        })
    };

    view! {
        <ConfirmButton
            location=RevLocation::SellConfirm
            on_click=sell_items
            disabled=no_cart_items
        >
            "SELL"
        </ConfirmButton>
    }
}

#[component]
fn ItemDetails() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let item = move || {
        let id = state.with(|state| state.modal_item_id);
        pc.with(|pc| pc.backpack.get(id).cloned().unwrap())
    };
    let stacks = move || {
        item()
            .find_counter()
            .map(|count| format!("Stack of {}.", count.max))
    };
    view! {
        <ModalCenter location=ModalLocation::ShopItemDetails>
            { move || item().into_view() }
            { stacks }
        </ModalCenter>
    }
}
