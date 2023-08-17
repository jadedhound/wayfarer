use leptos::*;

use crate::items::Item;
use crate::pc::inventory::stack_btn::stack_btn;
use crate::pc::PC;
use crate::svg;
use crate::utils::{expect_rw, some_if};

struct ChangeSlot(usize);

pub(super) fn quick_access() -> impl IntoView {
    provide_context(create_rw_signal(ChangeSlot(0)));
    let slots = move || {
        expect_rw::<PC>().with(|pc| {
            pc.quick_access
                .iter()
                .enumerate()
                .map(|(id, item)| quick_slot(id, item))
                .collect_view()
        })
    };

    view! {
        <div class= "flex flex-col shaded-table">
            { slots }
        </div>
    }
}

fn quick_slot(id: usize, item: &Option<Item>) -> impl IntoView {
    let pc = expect_rw::<PC>();
    let has_stacks = item.as_ref().is_some_and(|x| x.stacks.is_some());
    let stacks = some_if(has_stacks).map(|_| stack_btn(id, false));
    let remove_disabled = move || pc.with(|pc| pc.quick_access[id].is_none());
    let remove = move || {
        pc.update(|pc| {
            if let Some(item) = pc.quick_access[id].clone() {
                pc.inventory.add(item);
                pc.quick_access[id] = None
            }
        });
    };
    let svg_class = if item.is_some() {
        "stroke-red-800"
    } else {
        "stroke-zinc-500"
    };
    let item = match item {
        Some(item) => item.into_view(),
        None => view! {
            <div class= "py-2 text-center"> "Empty" </div>
        }
        .into_view(),
    };

    view! {
        <div class= "flex gap-x-2">
            <div class= "w-full p-2"> { item } </div>
            { stacks }
            <button
                class= "px-2"
                on:click=move |_| remove()
                disabled=remove_disabled
            >
                <div class=format!("w-4 {svg_class}") inner_html=svg::CROSS />
            </button>
        </div>
    }
}
