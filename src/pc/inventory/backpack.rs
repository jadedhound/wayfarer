use leptos::*;
use leptos_router::A;

use crate::icons;
use crate::items::meta::EMPTY_ITEM;
use crate::items::ItemProp;
use crate::pc::inventory::count_button::count_button;
use crate::pc::session::Session;
use crate::pc::{Ability, PC};
use crate::utils::concat_if;
use crate::utils::rw_utils::RwUtils;
use crate::views::revealer::{RevLocation, Revealer};
use crate::views::wealth::maybe_wealth;

pub fn backpack() -> impl IntoView {
    let sesh = Session::expect();
    let id_list = move || sesh.with(|sesh| sesh.sorted_inv.clone());

    view! {
        { fatigue }
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

fn fatigue() -> impl IntoView {
    let pc = PC::expect();
    let fatigue = PC::slice(|pc| pc.fatigue);
    let is_locked = RwSignal::new(true);
    #[rustfmt::skip]
    let lock_icon = move || {
        if is_locked.get() { icons::LOCKED } else { icons::UNLOCKED }
    };
    #[rustfmt::skip]
    let lock_colour = move || {
        if is_locked.get() { "fill-zinc-500" } else { "fill-yellow-500" }
    };

    view! {
        <div class= "flex gap-4 border-y-2 border-orange-600 py-2">
            <button
                class=move || format!("ml-2 w-5 {}", lock_colour())
                on:click=move |_| is_locked.update(|x| *x = !*x)
                inner_html=lock_icon
            />
            <div class= "w-12 grow">
                <h6> "Fatigue" </h6>
                <div class= "italic">
                    "Each point of fatigue reduces the available inventory."
                </div>
            </div>
            <button
                class= "w-5 rotate-180 disabled:invisible"
                on:click=move |_| pc.update(|pc| pc.fatigue -= 1)
                disabled=move || { fatigue.get() < 1 || is_locked.get() }
                inner_html=icons::RIGHT_CHEV
            />
            <h5 class= "self-center"> { fatigue } </h5>
            <button
                class= "w-5 disabled:invisible"
                on:click=move |_| pc.update(|pc| pc.fatigue += 1)
                disabled=move || { fatigue.get() > 9 || is_locked.get() }
                inner_html=icons::RIGHT_CHEV
            />
        </div>
    }
}

/// Shows empty slots for a PC.
fn empty_slots() -> impl IntoView {
    let sesh = Session::expect();
    let empty = move |i| {
        view! {
            <div class= "flex">
                <div class= "w-12 flex-center"> { i } </div>
                <div class= "psuedo h-20 w-12 grow" />
            </div>
        }
    };

    move || {
        sesh.with(|sesh| {
            let max_inv = sesh.abi_scores.get(Ability::MaxInventory) as usize;
            (sesh.empty_inv_slots > 0).then(|| {
                (max_inv - sesh.empty_inv_slots + 1..=max_inv)
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
    let (pc, sesh) = (PC::expect(), Session::expect());
    let is_quick = PC::slice(move |pc| pc.quick_access.iter().any(|x| x == &id));
    let disabled = PC::slice(move |pc| pc.quick_access.is_full() && !is_quick.get_untracked());
    let toggle_quick = move |_| {
        if is_quick.get() {
            pc.update(|pc| {
                pc.quick_access.remove_where(|x| *x == id);
            });
        } else {
            pc.update(|pc| pc.quick_access.push(id));
        }
    };
    let icon_or_range = move || {
        if disabled.get() {
            let range = sesh.with(|sesh| sesh.inv_slots.get(id).copied().unwrap_or_default());
            view! {
                <div class= "text-center">
                    { range }
                </div>
            }
            .into_view()
        } else {
            view! {
                <div class= "w-4" inner_html=icons::STAR />
            }
            .into_view()
        }
    };

    view! {
        <button
            class=concat_if(
                move || is_quick.get(),
                "stroke-yellow-500 fill-transparent w-12 flex-center",
                "fill-yellow-500"
            )
            on:click=toggle_quick
            disabled=disabled
        >
            { icon_or_range }
        </button>
    }
}

fn more_button(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let show_delete_modal = move |_| {
        Revealer::hide();
        pc.update(|pc| pc.inventory_remove(id))
    };
    let copy_item = move |_| {
        Revealer::hide();
        pc.update(|pc| {
            if let Some(item) = pc.inventory.get(id).cloned() {
                pc.inventory.add(item)
            }
        })
    };
    let cannot_use = pc.with_untracked(|pc| {
        !pc.inventory
            .get(id)
            .map(|item| {
                item.props
                    .iter()
                    .any(|prop| matches!(prop, ItemProp::Buff(_) | ItemProp::Usable(_)))
            })
            .unwrap_or(false)
    });
    let use_item = move |_| pc.update(|pc| pc.use_item(id));
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
