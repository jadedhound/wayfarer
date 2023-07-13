use leptos::*;

use crate::items::Item;
use crate::pc::inventory::inv_item::InvItem;
use crate::pc::PC;
use crate::utils::{read_context};

/// Filters out equiped items.
fn not_equiped_items(pc: &PC) -> Vec<Item> {
    let mut result = Vec::new();
    let equiped: Vec<&usize> = pc.equipment.0.iter().flatten().collect();
    for (i, item) in pc.inventory.iter().enumerate() {
        if !equiped.contains(&&i) {
            result.push(item.clone())
        }
    }
    result
}

#[component]
pub fn Backpack(cx: Scope) -> impl IntoView {
    let pc = read_context::<PC>(cx);
    let backpack_view = move |pc: &PC| {
        let mut counter = 1;
        let heavy = move |start, end| {
            (start..end)
                .map(|i| {
                    view! {
                        cx,
                        <InvIndex i />
                    }
                })
                .collect_view(cx)
        };

        not_equiped_items(pc)
            .into_iter()
            .map(|item| {
                let weight = item.weight();
                counter += weight;
                view! {
                    cx,
                    <InvIndex i={counter - weight} />
                    <BackpackItem item />
                    { move || heavy(counter - weight + 1, counter)}
                }
            })
            .collect_view(cx)
    };

    view! {
        cx,
        <h4 class= "border-b-2 border-sky-700 text-center"> "Backpack" </h4>
        <div class= "grid grid-cols-8 grid-flow-row gap-2 mt-4">
            { move || pc.with(backpack_view)}
        </div>
        <button class= "text-center rounded bg-sky-900 w-full py-1 mt-4">
            "Add Item"
        </button>
        <div class= "psuedo h-6" />
    }
}

#[component]
fn InvIndex(cx: Scope, i: u8) -> impl IntoView {
    view! {
        cx,
        <div class= "border-sky-900 border-2 rounded flex-centered h-10 h-full">
            {i}
        </div>
    }
}

#[component]
fn BackpackItem(cx: Scope, item: Item) -> impl IntoView {
    let weight = item.weight();
    let span = if weight == 2 {
        "row-span-2"
    } else if weight == 3 {
        "row-span-3"
    } else {
        ""
    };
    let class = format!("bg-zinc-800 rounded col-span-7 {span}");

    view! {
        cx,
        <div class=class>
            <InvItem item />
        </div>
    }
}
