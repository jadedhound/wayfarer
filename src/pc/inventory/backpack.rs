use leptos::*;
use leptos_router::A;

use crate::icons;
use crate::items::meta::EMPTY_ITEM;
use crate::items::ItemProp;
use crate::pc::inventory::count_button::count_button;
use crate::pc::session::Session;
use crate::pc::{MAX_INVENTORY, PC};
use crate::utils::rw_utils::RwUtils;
use crate::views::delete_confirm::DeleteModal;
use crate::views::revealer::{RevLocation, Revealer};
use crate::views::wealth::maybe_wealth;

pub fn backpack() -> impl IntoView {
    let pc = PC::expect();
    let id_list = move || pc.with(|pc| pc.inventory.keys().collect::<Vec<usize>>());

    view! {
        <div class= "flex flex-col shaded-table">
            <For
                each=id_list
                key=|id| *id
                children=item_view
            />
            { empty_slots }
        </div>
    }
}

/// Shows empty slots for a PC.
fn empty_slots() -> impl IntoView {
    let pc = PC::expect();
    let empty = move |i| {
        view! {
            <div class= "flex">
                <div class= "w-12 flex-center"> { i } </div>
                <div class= "psuedo h-20 w-12 grow" />
            </div>
        }
    };

    move || {
        pc.with(|pc| {
            pc.inventory
                .size()
                .filter(|&amount| amount > 0)
                .map(|curr| {
                    (curr + 1..=pc.inventory.max_size())
                        .map(empty)
                        .collect_view()
                })
        })
    }
}

/// Renders the item with the `id` given.
fn item_view(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let item = pc.with_untracked(|pc| pc.inventory.get(id).cloned().unwrap_or(EMPTY_ITEM.into()));
    let stacks = item.find_counter().map(|_| count_button(id));
    let item_view = item.into_view();
    let price = move || {
        let price = pc
            .with(|pc| pc.inventory.get(id).map(|item| item.price()))
            .unwrap_or_default();
        maybe_wealth(price)
    };

    view! {
        <div class= "relative">
            <div class= "flex gap-2 w-full items-stretch">
                { slot_by_weight(id) }
                <div class= "py-2 w-12 grow flex flex-col">
                    { item_view }
                    <div class= "flex items-center justify-between flex-wrap">
                        { price }
                        { stacks }
                    </div>
                </div>
                { more_button(id) }
            </div>
        </div>
    }
}

fn slot_by_weight(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let slot = move || pc.with(|pc| pc.inventory.get_slot(id)).into_view();
    view! {
        <div class= "w-12 flex-center">
            { slot }
        </div>
    }
}

fn more_button(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let show_delete_modal = move |_| {
        Revealer::hide();
        pc.update(|pc| {
            if let Some(item) = pc.inventory.remove(id) {
                pc.recently_removed.push_unique(item);
            }
        })
    };
    let copy_item = move |_| {
        Revealer::hide();
        pc.update(|pc| {
            if let Some(item) = pc.inventory.get(id).cloned() {
                pc.inventory.add(item);
            }
        })
    };
    let cannot_use = pc.with_untracked(|pc| {
        !pc.inventory
            .get(id)
            .map(|item| {
                item.props
                    .iter()
                    .any(|prop| matches!(prop, ItemProp::Usable(_)))
            })
            .unwrap_or(false)
    });
    let use_item = move |_| {
        pc.update(|pc| {
            if let Some(item) = pc.inventory.use_item(id) {
                pc.recently_removed.push_unique(item);
            }
        })
    };
    let menu_hidden = create_memo(move |_| Revealer::is_hidden(RevLocation::BackpackMore, id));

    view! {
        <div class= "flex-center">
            <div class= "relative">
                <button
                    class= "px-2"
                    on:click=move |_| Revealer::show(RevLocation::BackpackMore, id)
                >
                    <div class= "w-6" inner_html=icons::ELLIPSES />
                </button>
                <div
                    class= "btn bg-surface flex flex-col z-40 w-28
                    absolute right-0 -translate-x-2 [&>*]:py-3"
                    hidden=menu_hidden
                >
                    <button class= "text-red-500" on:click=show_delete_modal>
                        "DELETE"
                    </button>
                    <A class= "text-center" href=format!("../edit_item/{id}")>
                        "EDIT"
                    </A>
                    <button hidden=cannot_use on:click=use_item> "USE" </button>
                    <button on:click=copy_item> "COPY" </button>
                </div>
            </div>
        </div>
    }
}
