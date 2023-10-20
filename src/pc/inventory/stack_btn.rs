use const_format::concatcp;
use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::concat_if;
use crate::utils::rw_utils::RwUtils;
use crate::views::revealer::Revealer;

const CHG_BTN: &str =
    "bg-black flex-center z-50 border-y-2 border-sky-800 h-10 w-10 relative -mx-2 disabled:invisible";

pub fn stack_btn(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let hide_btns = create_memo(move |_| !Revealer::is_shown('s', id));
    let count = PC::slice(move |pc| {
        pc.inventory
            .get(id)
            .and_then(|x| x.find_counter())
            .copied()
            .unwrap_or_default()
    });
    let curr = move || count.get().curr;
    let max = move || count.get().max;
    let cannot_add = move || hide_btns.get() || curr() >= max();
    let cannot_remove = move || hide_btns.get() || curr() < 2;
    let change_stack = move |by: i64| {
        pc.update(|pc| {
            if let Some(item) = pc.inventory.get_mut(id) {
                if let Some(count) = item.find_mut_counter() {
                    count.curr = (count.curr as i64 + by) as usize;
                }
            }
        })
    };

    view! {
        <div class= "flex justify-center">
            <button
                class=concatcp!(CHG_BTN, " border-l-2 rounded-l translate-x-2")
                on:click=move |_| change_stack(-1)
                disabled=cannot_remove
            >
                <div class= "w-4" inner_html=icons::MINUS />
            </button>
            <button
                class=concat_if(
                    move || !hide_btns.get(),
                    "px-4 h-10 flex items-center gap-2 border-2 border-transparent rounded",
                    "bg-black !border-sky-800"
                )
                on:click=move |_| { Revealer::show('s', id) }
            >
                <div class= ""> { move || format!("{} / {}", curr(), max())} </div>
                <div class= "w-4" inner_html=icons::STACK />
            </button>
            <button
                class=concatcp!(CHG_BTN, " border-r-2 rounded-r -translate-x-2")
                on:click=move |_| change_stack(1)
                disabled=cannot_add
            >
                <div class= "w-4" inner_html=icons::PLUS />
            </button>
        </div>
    }
}
