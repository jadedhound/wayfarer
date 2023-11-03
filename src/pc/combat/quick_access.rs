use leptos::*;

use crate::items::ItemProp;
use crate::pc::combat::use_button::use_button;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

pub(super) fn quick_access() -> impl IntoView {
    let pc = PC::expect();
    let no_items = PC::slice(|pc| pc.quick_access.is_empty());
    let quick_ids = PC::slice(|pc| pc.quick_access.clone());
    let quick_items = move || {
        quick_ids
            .get()
            .iter()
            .copied()
            .map(item_view)
            .collect_view()
    };
    let passive_ids = PC::slice(|pc| {
        pc.inventory
            .iter()
            .flat_map(|(id, item)| {
                item.props
                    .iter()
                    .any(|prop| matches!(prop, ItemProp::Passive))
                    .then_some(id)
            })
            .collect::<Vec<usize>>()
    });
    let passive_items = move || {
        let mut id_list = passive_ids.get();
        id_list.sort_unstable_by(|a, b| {
            let get_counter = move |id: usize| {
                pc.with_untracked(|pc| {
                    pc.inventory
                        .get(id)
                        .map(|item| item.find_counter().unwrap_or_default().max)
                })
            };
            let a = get_counter(*a);
            let b = get_counter(*b);
            a.cmp(&b)
        });
        id_list.into_iter().rev().map(item_view).collect_view()
    };

    view! {
        <div
            class= "text-center italic p-2"
            hidden=move || !no_items.get()
        >
            "Star an item to access it quickly in combat."
        </div>
        <div class= "flex flex-col shaded-table empty:hidden">
            { quick_items }
            { passive_items }
        </div>
    }
}

fn item_view(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let item = PC::expect().with(|pc| pc.inventory.get(id).cloned().unwrap_or_default());
    let can_use = item
        .props
        .iter()
        .any(|prop| matches!(prop, ItemProp::Usable(_) | ItemProp::Buff(_)));
    let find_counter = move || {
        pc.with(|pc| {
            pc.inventory
                .get(id)
                .and_then(|item| item.find_counter())
                .unwrap_or_default()
        })
    };
    let use_button = can_use
        .then(move || use_button(find_counter, move || pc.update(|pc| pc.use_item(id)), true));
    view! {
        <div class= "flex flex-col gap-1 p-2">
            { item.into_view() }
            { use_button }
        </div>
    }
}
