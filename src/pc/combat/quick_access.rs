use leptos::*;

use crate::items::ItemProp;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

pub(super) fn quick_access() -> impl IntoView {
    let pc = PC::expect();
    let no_items = PC::slice(|pc| pc.quick_access.is_empty());
    let quick_items = move || pc.with(|pc| pc.quick_access.iter().map(quick_item).collect_view());

    view! {
        <div
            class= "text-center italic p-2"
            hidden=move || !no_items.get()
        >
            "Star an item to access it quickly in combat."
        </div>
        <div
            class= "flex flex-col shaded-table"
            hidden=no_items
        >
            { quick_items }
        </div>
    }
}

fn quick_item(id: &usize) -> impl IntoView {
    let item = PC::expect().with(|pc| pc.inventory.get(*id).cloned().unwrap_or_default());
    let use_btn = item
        .props
        .iter()
        .find(|prop| matches!(prop, ItemProp::Usable(_) | ItemProp::Buff(_)))
        .map(|_| use_item_btn(*id));
    view! {
        <div class= "flex flex-col gap-1 p-2">
            { item.into_view() }
            { use_btn }
        </div>
    }
}

fn use_item(pc: &mut PC, id: usize) {
    let item = pc.inventory.get_mut(id).unwrap();
    for prop in item.props.iter() {
        if let ItemProp::Buff(buff) = prop {
            pc.buffs.add(buff.clone())
        }
    }
    if let Some(count) = item.find_mut_counter() {
        count.curr -= 1;
        if count.is_empty() {
            pc.quick_access.remove_where(|x| *x == id);
            pc.inventory.remove(id);
        }
    } else {
        pc.quick_access.remove_where(|x| *x == id);
        pc.inventory.remove(id);
    }
}

fn use_item_btn(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let uses_left = move || {
        pc.with(|pc| {
            pc.inventory
                .get(id)
                .and_then(|item| item.find_counter().map(|count| count.curr))
                .unwrap_or(1)
        })
    };
    let on_click = move |_| pc.update(|pc| use_item(pc, id));

    view! {
        <div class= "flex gap-1">
            <div class= "border-2 border-green-600 rounded py-2 w-12 text-center font-tight">
                <div> { uses_left } </div>
            </div>
            <button
                class= "w-12 grow btn bg-green-800 py-2"
                on:click=on_click
            >
                "USE"
            </button>
        </div>
    }
}
