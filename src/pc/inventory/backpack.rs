use leptos::*;

use crate::icons;
use crate::items::{Item, ItemProp};
use crate::pc::inventory::stack_btn::stack_btn;
use crate::pc::session::{PCSession, SlotRange};
use crate::pc::PC;
use crate::utils::{some_if, RwProvided};
use crate::views::delete_btn::delete_btn;
use crate::views::funds::short_funds;
use crate::views::revealer::Revealer;

pub fn backpack() -> impl IntoView {
    let id_list = move || {
        PC::with(|pc| {
            pc.inventory
                .iter()
                .map(|(id, item)| (id, item.clone()))
                .collect::<Vec<_>>()
        })
    };

    view! {
        <div class= "flex flex-col shaded-table">
            <For
                each=id_list
                key=|(id, _)| *id
                children=item_view
            />
            { empty_slots() }
        </div>
    }
}

/// Shows empty slots for a PC.
fn empty_slots() -> impl IntoView {
    let empty = move |i| {
        view! {
            <div class= "flex">
                <div class= "w-12 flex-center"> { i } </div>
                <div class= "psuedo h-20 w-12 grow" />
            </div>
        }
    };
    let slot_to_curr = |slot: &SlotRange| match slot {
        SlotRange::Single(x) => Some(*x),
        SlotRange::Double(x) => Some(*x + 1),
        SlotRange::Encumbered => None,
    };

    move || {
        PCSession::with(|sesh| {
            let max = sesh.max_inv;
            let curr = sesh
                .inv_slots
                .values()
                .last()
                .and_then(slot_to_curr)
                .unwrap_or(max);
            some_if(curr <= max).map(|_| (curr + 1..=max).map(empty).collect_view())
        })
    }
}

/// Renders the item with the `id` given.
fn item_view((id, item): (usize, Item)) -> impl IntoView {
    let range = move || PCSession::with(|sesh| sesh.inv_slots.get(id).cloned().unwrap_or_default());
    let price = short_funds(move || item.price);
    // TODO: Should only read counter once.
    let stacks = some_if(item.find_counter().is_some()).map(|_| stack_btn(id, true));
    let delete_item = move || {
        Revealer::hide();
        PC::update(|pc| pc.inventory.remove(id));
    };

    view! {
        <div class= "relative">
            <button
                class= "flex gap-2 w-full items-stretch"
                on:contextmenu=move |event| {
                    event.prevent_default();
                    Revealer::show('d', id);
                }
            >
                <div class= "w-12 flex-center"> { range } </div>
                <div class= "py-2 w-12 grow flex flex-col">
                    { item.into_view() }
                    { stacks }
                    { price }
                </div>
                { more_btn(id, item.clone()) }
            </button>
            { delete_btn('d', id, delete_item) }
        </div>
    }
}

const MORE_BTN_CSS: &str = "w-24 py-2 disabled:text-zinc-500";

fn more_btn(id: usize, item: Item) -> impl IntoView {
    let menu_is_shown = move || Revealer::state('m', id);
    let popup = move || {
        some_if(menu_is_shown()).map(|_| {
            view! {
                <div class= "absolute translate-y-1 -translate-x-[5.5rem] btn bg-surface
                            flex flex-col divide-y divide-dashed px-2 absolute rounded z-40">
                    { add_to_quick(id) }
                    { use_item_btn(id, &item) }
                </div>
            }
        })
    };

    view! {
        <button class= "flex-center w-8"
            on:click=move |_| Revealer::show('m', id)
        >
            <div class= "relative">
                <div class= "w-6 icons" inner_html=icons::VERT_ELLIPS />
                { popup }
            </div>
        </button>
    }
}

fn add_to_quick(id: usize) -> impl IntoView {
    let to_quick = move || {
        Revealer::hide();
        PC::update(|pc| {
            pc.inventory.remove(id).and_then(|item| {
                let i = pc.quick_access.iter().position(|x| x.is_none())?;
                pc.quick_access[i] = Some(item);
                Some(())
            });
        });
    };
    let to_quick_disabled = move || {
        PC::with(|pc| {
            let curr = pc.quick_access.iter().flatten().count();
            curr >= pc.quick_access.len()
        })
    };
    view! {
            <button
                class=MORE_BTN_CSS
                on:click=move |_| to_quick()
                disabled=to_quick_disabled
            >
                "QUICK ACCESS"
            </button>
    }
}

fn use_item_btn(id: usize, item: &Item) -> impl IntoView {
    let mut buff = None;
    let mut cannot_use = true;
    for prop in item.props.iter() {
        match prop {
            ItemProp::Usable(_) => cannot_use = false,
            ItemProp::Buff(x) => {
                buff = Some(x.clone());
                cannot_use = false;
            }
            _ => (),
        }
    }

    let decr_item = move |pc: &mut PC| {
        let item = pc.inventory.get_mut(id).unwrap();
        if let Some(count) = item.find_mut_counter() {
            count.decr();
            if count.is_zero() {
                pc.inventory.remove(id);
            }
        } else {
            pc.inventory.remove(id);
        }
    };
    let use_item = move |pc: &mut PC| {
        if let Some(buff) = buff.clone() {
            pc.buffs.add(buff);
        }
    };
    let on_click = move || {
        Revealer::hide();
        PC::update(|pc| {
            use_item(pc);
            decr_item(pc)
        })
    };

    view! {
        <button
            class= MORE_BTN_CSS
            on:click=move |_| on_click()
            hidden=cannot_use
        >
            "USE"
        </button>
    }
}
