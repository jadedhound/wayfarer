use leptos::*;
use leptos_router::use_location;

use super::Shop;
use crate::icons;
use crate::items::{Item, ItemRef};
use crate::pc::session::Session;
use crate::pc::PC;
use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;
use crate::utils::{expect_rw, RwSignalEnhance};
use crate::views::funds::{maybe_funds, short_funds};
use crate::views::modal::{ModalCenter, ModalState};
use crate::views::revealer::Revealer;

#[derive(Clone, Default)]
struct State {
    cart: IndexMap<&'static ItemRef>,
    price: u32,
    weight: usize,
    item_details: &'static ItemRef,
}

impl RwUtils for State {
    type Item = State;
}

#[component]
pub fn ShopView() -> impl IntoView {
    let shop = location_to_shop();
    let state = State::provide();
    state_updater();
    let shop_items = move || shop.items().iter().map(shop_item).collect_view();
    let cart = move || {
        state.with(|x| {
            x.cart
                .iter()
                .map(|(id, item)| (id, *item))
                .collect::<Vec<_>>()
        })
    };

    view! {
        <h3 class= "text-center"> { shop.to_string() } </h3>
        <div class= "italic text-center"> { shop.desc() } </div>
        <div class= "flex flex-col shaded-table">
            { shop_items }
        </div>
        <h3 class= "text-center"> { "Cart" } </h3>
        <div
            class= "flex flex-col shaded-table"
            hidden=move || state.with(|state| state.cart.is_empty())
        >
            <For
                each=cart
                key=|(id, _)| *id
                children=cart_item
            />
        </div>
        { price }
        { purchase_button }
        { modal_item_details }
    }
}

fn location_to_shop() -> Shop {
    let path = use_location().pathname.get_untracked();
    path.split('/')
        .last()
        .and_then(|last_word| {
            let i = last_word.parse::<usize>().ok()?;
            Shop::from_repr(i)
        })
        .unwrap_or_default()
}

fn state_updater() {
    let state = State::expect();
    let price = create_read_slice(state, |state| {
        state.cart.values().map(|x| x.price()).sum::<u32>()
    });
    create_effect(move |_| price.with(|price| state.update(|x| x.price = *price)));
    let weight = State::slice(|state| {
        state
            .cart
            .values()
            .map(|item| item.is_bulky() as usize + 1)
            .sum::<usize>()
    });
    create_effect(move |_| weight.with(|weight| state.update(|x| x.weight = *weight)));
}

fn cart_item((id, item_ref): (usize, &'static ItemRef)) -> impl IntoView {
    let state = State::expect();
    let remove_from_cart = move || state.update_discard(|x| x.cart.remove(id));
    name_and_price(item_ref, remove_from_cart)
}

fn shop_item(item_ref: &&'static ItemRef) -> impl IntoView {
    let item_ref = *item_ref;
    let state = State::expect();
    let add_to_cart = move || state.update(|x| x.cart.add(item_ref));
    name_and_price(item_ref, add_to_cart)
}

fn name_and_price<F>(item_ref: &'static ItemRef, on_click: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let state = State::expect();
    let open_details = move |_| {
        ModalState::show(10);
        state.update(|x| x.item_details = item_ref);
    };
    view! {
        <div class= "p-2 flex gap-2">
            <button
                class= "w-4 stroke-sky-500"
                on:click=open_details
                hidden=move || item_ref.props.is_empty()
                inner_html=icons::INFO
            />
            <button
                class= "capitalise w-12 grow text-left"
                on:click= move |_| on_click()
            >
                { item_ref.name }
            </button>
            { maybe_funds(item_ref.price()) }
        </div>
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
            short_funds(funds_left).into_view()
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
            { move || short_funds(price.get()) }
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
                class= "btn bg-surface p-2 w-full"
                on:click=move |_| Revealer::show('p', 0)
                disabled=cannot_buy
            >
                "PURCHASE"
            </button>
            <div hidden=move || Revealer::hidden('p', 0)>
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

fn modal_item_details() -> impl IntoView {
    let state = expect_rw::<State>();
    let item = move || state.with(|x| Item::from(*x.item_details));
    let item_view = move || item().into_view();
    let stacks = move || {
        item()
            .find_counter()
            .map(|count| format!("Stack of {}.", count.max))
    };

    view! {
        <ModalCenter id=10>
            { item_view }
            { stacks }
        </ModalCenter>
    }
}
