use const_format::concatcp;
use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::{some_if, RwProvided};
use crate::views::revealer::Revealer;

const CHG_BTN: &str =
    "bg-black flex-center z-50 border-y-2 border-sky-800 h-10 w-10 relative disabled:invisible";

pub fn stack_btn(id: usize, is_inventory: bool) -> impl IntoView {
    let btn_id = if is_inventory { 'i' } else { 'q' };
    let hide_btns = create_memo(move |_| !Revealer::state(btn_id, id));
    let count = PC::slice(move |pc| {
        if is_inventory {
            pc.inventory
                .get(id)
                .and_then(|x| x.find_counter())
                .copied()
                .unwrap_or_default()
        } else {
            pc.quick_access[id]
                .as_ref()
                .and_then(|x| x.find_counter())
                .copied()
                .unwrap_or_default()
        }
    });
    let curr = move || count.get().curr;
    let max = move || count.get().max;
    let active_css = move || {
        some_if(!hide_btns.get())
            .map(|_| "bg-black border-sky-800 border-2 rounded")
            .unwrap_or_default()
    };
    let cannot_add = move || hide_btns.get() || curr() >= max();
    let cannot_remove = move || hide_btns.get() || curr() < 2;

    view! {
        <div class= "flex justify-center">
            <button
                class=concatcp!(CHG_BTN, " border-l-2 rounded-l translate-x-2")
                on:click=move |_| change_stack(id, is_inventory, -1)
                disabled=cannot_remove
            >
                <div class= "w-4" inner_html=icons::MINUS />
            </button>
            <button
                class=move || format!("text-sky-500 px-4 h-10 {}", active_css())
                on:click=move |_| { Revealer::show(btn_id, id) }
            >
                { move || format!("{} / {}", curr(), max())}
            </button>
            <button
                class=concatcp!(CHG_BTN, " border-r-2 rounded-r -translate-x-2")
                on:click=move |_| change_stack(id, is_inventory, 1)
                disabled=cannot_add
            >
                <div class= "w-4" inner_html=icons::PLUS />
            </button>
        </div>
    }
}

/// Changes the stack of an item in the quick access slot.
fn change_stack(id: usize, is_inventory: bool, by: i64) {
    PC::update(|pc| {
        let item = if is_inventory {
            pc.inventory.get_mut(id)
        } else {
            pc.quick_access[id].as_mut()
        };
        if let Some(item) = item {
            if let Some(count) = item.find_mut_counter() {
                count.curr = (count.curr as i64 + by) as usize;
            }
        }
    })
}
