use leptos::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

use crate::icons;
use crate::items::{self, Item, ItemPropRef, ItemRef};
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::utils::index_map::IndexMap;
use crate::utils::{expect_rw, some_if, RwProvided};
use crate::views::funds::{maybe_funds, short_funds};
use crate::views::modal::{ModalCenter, ModalState};
use crate::views::revealer::Revealer;

#[derive(Serialize, Deserialize, Clone, Copy, EnumIter, Display, Default)]
pub enum Shop {
    #[default]
    Alchemist,
    #[strum(serialize = "Arcane Workshop")]
    Arcane,
    Blacksmith,
    #[strum(serialize = "Adventuring Supplies")]
    Adventurer,
    #[strum(serialize = "Holy Ground")]
    Holy,
}

impl Shop {
    fn item_arr(&self) -> [&[&'static ItemRef]; 1] {
        match self {
            Shop::Alchemist => [&items::SHOP_ALCHEMY_T1],
            Shop::Arcane => [&items::SHOP_ARCANE_T1],
            Shop::Blacksmith => [&items::SHOP_SMITH_T1],
            Shop::Adventurer => [&items::SHOP_ADVENTURE_T1],
            Shop::Holy => [&items::SHOP_HOLY_T1],
        }
    }
}

#[derive(Clone, Default)]
struct ShopState {
    rank: usize,
    cart: IndexMap<&'static ItemRef>,
    price: u32,
    weight: usize,
    item_details: &'static ItemRef,
}

pub fn shop() -> impl IntoView {
    let shop = PCSession::with(|sesh| sesh.active_shop);
    let state = RwSignal::new(ShopState::default());
    let rank = create_read_slice(state, |state| state.rank);
    provide_context(state);
    state_updater();
    let shop_items = move || {
        shop.item_arr()[rank.get()]
            .iter()
            .map(shop_btn)
            .collect_view()
    };
    let cart = move || {
        state.with(|x| {
            x.cart
                .iter()
                .map(|(id, item)| (id, *item))
                .collect::<Vec<_>>()
        })
    };
    let cart_wrapper = move || {
        some_if(state.with(|x| !x.cart.is_empty())).map(|_| {
            view! {
                <div class= "flex flex-col shaded-table">
                    <For
                        each=cart
                        key=|(id, _)| *id
                        children=cart_btn
                    />
                </div>
            }
        })
    };

    view! {
        <h3 class= "text-center"> { shop.to_string() } </h3>
        <div class= "flex flex-col shaded-table">
            { shop_items }
        </div>
        <h3 class= "text-center"> { "Cart" } </h3>
        { cart_wrapper }
        { price }
        { purchase_btn }
        { modal_item_details }
    }
}

fn state_updater() {
    let state = expect_rw::<ShopState>();
    let price = create_read_slice(state, |state| {
        state.cart.values().map(|x| x.price).sum::<u32>()
    });
    create_effect(move |_| price.with(|price| state.update(|x| x.price = *price)));
    let weight = create_read_slice(state, |state| {
        state
            .cart
            .values()
            .map(|x| {
                let is_bulky = x
                    .props
                    .iter()
                    .any(|prop| matches!(prop, ItemPropRef::Bulky));
                is_bulky as usize + 1
            })
            .sum::<usize>()
    });
    create_effect(move |_| weight.with(|weight| state.update(|x| x.weight = *weight)));
}

fn item_ref_view<F>(item_ref: &'static ItemRef, on_click: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let state = expect_rw::<ShopState>();
    let open_details = move |_| {
        ModalState::open(0);
        state.update(|x| x.item_details = item_ref);
    };
    let question_mark = some_if(!item_ref.props.is_empty()).map(|_| {
        view! {
            <button
                class= "btn !rounded-full p-1 bg-blue-800"
                on:click=open_details
            >
                <div class= "w-3" inner_html=icons::QUESTION_MARK />
            </button>
        }
    });
    view! {
        <div class= "p-2 flex gap-2">
            { question_mark }
            <button
                class= "capitalise w-12 grow text-left"
                on:click= move |_| on_click()
            >
                { item_ref.name }
            </button>
            { maybe_funds(item_ref.price) }
        </div>
    }
}

fn shop_btn(item_ref: &&'static ItemRef) -> impl IntoView {
    let item_ref = *item_ref;
    let state = expect_rw::<ShopState>();
    let add_to_cart = move || state.update(|x| x.cart.add(item_ref));
    item_ref_view(item_ref, add_to_cart)
}

fn cart_btn((id, item_ref): (usize, &'static ItemRef)) -> impl IntoView {
    let state = expect_rw::<ShopState>();
    let remove_from_cart = move || {
        state.update(|x| {
            x.cart.remove(id);
        })
    };

    item_ref_view(item_ref, remove_from_cart)
}

fn price() -> impl IntoView {
    let state = expect_rw::<ShopState>();
    let pc_funds = PC::with(|pc| pc.funds);
    let price = create_read_slice(state, |state| state.price);
    let empty_inv = PCSession::with(|sesh| sesh.empty_inv_slots);
    let weight = create_read_slice(state, |state| state.weight);
    let funds_left = move || {
        if price.get() > pc_funds {
            view! {
                <div class= "font-tight text-red-500"> "Not enough funds" </div>
            }
            .into_view()
        } else {
            short_funds(pc_funds - price.get()).into_view()
        }
    };
    let weight_left = move || {
        if weight.get() > empty_inv {
            view! {
                <div class= "font-tight text-red-500"> "Encumbered" </div>
            }
            .into_view()
        } else {
            (empty_inv - weight.get()).into_view()
        }
    };

    view! {
        <div class= "grid grid-cols-2 items-center gap-x-2 border-y-2 border-yellow-500 py-2">
            <div class= "text-right font-tight"> "Price:" </div>
            { move || short_funds(price.get()) }
            <div class= "text-right font-tight"> "Funds left:" </div>
            { funds_left }
            <div class= "text-right font-tight"> "Inventory left:" </div>
            { weight_left }
        </div>
    }
}

fn purchase_btn() -> impl IntoView {
    let state = expect_rw::<ShopState>();
    let pc_funds = PC::with(|pc| pc.funds);
    let cannot_buy = create_read_slice(state, move |state| {
        state.cart.is_empty() || pc_funds < state.price
    });
    let buy_items = move |_| {
        let (items, price) = state.with(|x| {
            (
                x.cart.values().map(|x| Item::from(**x)).collect::<Vec<_>>(),
                x.price,
            )
        });
        state.update(|x| x.cart = IndexMap::default());
        PC::update(|pc| {
            items.into_iter().for_each(|x| pc.inventory.add(x));
            pc.funds -= price;
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
    let state = expect_rw::<ShopState>();
    let details = move || state.with(|x| Item::from(*x.item_details)).into_view();

    view! {
        <ModalCenter id=0>
            { details }
        </ModalCenter>
    }
}
