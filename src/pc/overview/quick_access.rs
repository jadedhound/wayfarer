use leptos::*;

use crate::items::{Item, ItemProp};
use crate::pc::PC;
use crate::utils::RwProvided;

pub(super) fn quick_access() -> impl IntoView {
    let no_items = PC::slice(|pc| pc.quick_access.iter().flatten().next().is_none());

    move || {
        if no_items.get() {
            view! {
                <div class= "text-center italic p-2">
                    "Items in quick access slots will be shown here."
                </div>
            }
            .into_view()
        } else {
            table().into_view()
        }
    }
}

fn table() -> impl IntoView {
    let quick_access = PC::untracked(|pc| {
        pc.quick_access
            .iter()
            .enumerate()
            .flat_map(|(id, item)| {
                let item = item.as_ref()?;
                Some(quick_item(id, item))
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class= "flex flex-col shaded-table">
            { quick_access }
        </div>
    }
}

fn quick_item(id: usize, item: &Item) -> impl IntoView {
    let use_btn = item
        .props
        .iter()
        .find(|prop| matches!(prop, ItemProp::Usable(_)))
        .map(|_| use_item_btn(id));
    view! {
        <div class= "">
            { item.into_view() }
            { use_btn }
        </div>
    }
}

fn use_item(id: usize) {
    PC::update(|pc| {
        let item = pc.quick_access[id].as_mut().unwrap();
        if let Some(count) = item.find_mut_counter() {
            count.decr();
            if count.is_zero() {
                pc.quick_access[id] = None;
            }
        } else {
            pc.quick_access[id] = None;
        }
    })
}

fn use_item_btn(id: usize) -> impl IntoView {
    let uses_left = move || {
        let curr = PC::with(|pc| {
            pc.quick_access[id]
                .as_ref()
                .and_then(|item| item.find_counter().map(|count| count.curr))
                .unwrap_or_default()
        });
        let uses = if curr > 2 { "uses" } else { "use" };
        format!("{curr} {uses} left.")
    };

    view! {
        <div class= "flex">
            <div class= "">
                <div> { uses_left } </div>
            </div>
            <button
                class= "w-12 grow btn bg-green-800"
                on:click=move |_| use_item(id)
            >
                "USE"
            </button>
        </div>
    }
}
