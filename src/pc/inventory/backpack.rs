use leptos::*;

use crate::icons;
use crate::items::{Item, ItemProp};
use crate::pc::inventory::stack_btn::stack_btn;
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::utils::{expect_rw, some_if, RwProvided};
use crate::views::delete_btn::{delete_btn, delete_btn_show};
use crate::views::funds::maybe_funds;
use crate::views::revealer::Revealer;

#[derive(Default)]
struct BackpackState {
    quick_full: bool,
}

impl RwProvided for BackpackState {
    type Item = BackpackState;
}

pub fn backpack() -> impl IntoView {
    let id_list = move || {
        PC::with(|pc| {
            pc.inventory
                .iter()
                .map(|(id, item)| (id, item.clone()))
                .collect::<Vec<_>>()
        })
    };
    provide_context(RwSignal::new(BackpackState::default()));
    update_state();

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

    move || {
        PCSession::with(|sesh| {
            some_if(sesh.empty_inv_slots > 0).map(|_| {
                (sesh.max_inv - sesh.empty_inv_slots + 1..=sesh.max_inv)
                    .map(empty)
                    .collect_view()
            })
        })
    }
}

/// Renders the item with the `id` given.
fn item_view((id, item): (usize, Item)) -> impl IntoView {
    let range = move || PCSession::with(|sesh| sesh.inv_slots.get(id).copied().unwrap_or_default());
    let price = move || maybe_funds(item.price);
    // TODO: Should only read counter once.
    let stacks = some_if(item.find_counter().is_some()).map(|_| stack_btn(id, true));
    let delete_item = move || PC::update(|pc| pc.inventory.remove(id));

    view! {
        <div class= "relative">
            <div class= "flex gap-2 w-full items-stretch" on:contextmenu=delete_btn_show('d', id)>
                <div class= "w-12 flex-center"> { range } </div>
                <div class= "py-2 w-12 grow flex flex-col">
                    { item.into_view() }
                    { stacks }
                    { price }
                </div>
                { add_to_quick(id) }
            </div>
            { delete_btn('d', id, delete_item) }
        </div>
    }
}

fn add_to_quick(id: usize) -> impl IntoView {
    let disabled = move || BackpackState::with(|x| x.quick_full);
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
    view! {
        <button
            class= "px-2 disabled:fill-zinc-500"
            on:click=move |_| to_quick()
            disabled=disabled

        >
            <div class= "w-4" inner_html=icons::STAR />
        </button>
    }
}

fn update_state() {
    let quick_full =
        PC::slice(|pc| pc.quick_access.iter().flatten().count() == pc.quick_access.len());

    create_effect(move |_| {
        let quick_full = quick_full.get();
        BackpackState::update(|x| x.quick_full = quick_full);
    });
}
