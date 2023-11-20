use const_format::concatcp;
use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::concat_if;
use crate::utils::rw_utils::RwUtils;
use crate::views::revealer::{RevLocation, Revealer};

const CHG_BTN: &str =
    "bg-black flex-center z-50 border-y-2 border-sky-800 h-10 w-10 relative -mx-2 disabled:invisible";

pub fn count_button(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let is_interactive = create_memo(move |_| Revealer::is_shown(RevLocation::CountButton, id));
    let is_non_interactive = move || !is_interactive.get();
    let count = PC::slice(move |pc| {
        pc.backpack
            .get(id)
            .and_then(|x| x.find_counter())
            .unwrap_or_default()
    });
    let curr = move || count.get().curr;
    let max = move || count.get().max;
    let cannot_add = move || is_non_interactive() || curr() >= max();
    let cannot_remove = move || is_non_interactive() || curr() < 2;
    let change_stack = move |by: i64| {
        pc.update(|pc| {
            let item_count = pc
                .backpack
                .get_mut(id)
                .and_then(|item| item.find_mut_counter());
            if let Some(count) = item_count {
                count.curr = (count.curr as i64 + by) as usize;
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
                    is_interactive.into(),
                    "px-4 h-10 flex items-center gap-2 border-2 border-transparent rounded",
                    "bg-black !border-sky-800",
                    "",
                )
                on:click=move |_| { Revealer::show(RevLocation::CountButton, id) }
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
