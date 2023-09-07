use leptos::*;

use crate::icons;
use crate::items::Item;
use crate::pc::inventory::stack_btn::stack_btn;
use crate::pc::PC;
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
    let has_stacks = item.as_ref().is_some_and(|x| x.find_counter().is_some());
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
    let item = match item {
        Some(item) => item.into_view(),
        None => view! {
            <div class= "py-2 text-center"> "Empty" </div>
        }
        .into_view(),
    };

    view! {
        <div class= "flex gap-x-2">
            <div class= "flex flex-col w-12 grow p-2">
                { item }
                { stacks }
            </div>
            <button
                class= "flex-center px-2 fill-red-500 disabled:fill-zinc-700"
                on:click=move |_| remove()
                disabled=remove_disabled
            >
                <div class= "w-4" inner_html=icons::CROSS />
            </button>
        </div>
    }
}
