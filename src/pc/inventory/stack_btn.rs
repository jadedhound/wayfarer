use const_format::concatcp;
use leptos::*;

use crate::pc::PC;
use crate::svg;
use crate::utils::{some_if, RwProvided};
use crate::views::revealer::Revealer;

pub fn stack_btn(id: usize, is_inventory: bool) -> impl IntoView {
    const CHG_BTN: &str =
        "bg-black absolute flex-centered w-10 h-10 z-50 border-x-2 border-sky-800";
    let btn_id = if is_inventory { 'i' } else { 'q' };
    let hide_btns = create_memo(move |_| !Revealer::state(btn_id, id));
    let stacks = move || {
        PC::with(|pc| {
            if is_inventory {
                pc.inventory
                    .get(id)
                    .and_then(|x| x.stacks)
                    .unwrap_or_default()
            } else {
                pc.quick_access[id]
                    .as_ref()
                    .and_then(|x| x.stacks)
                    .unwrap_or_default()
            }
        })
    };
    let curr = move || stacks().0;
    let max = move || stacks().1;
    let active_css = move || {
        some_if(!hide_btns.get())
            .map(|_| "bg-black border-sky-800 rounded border-2")
            .unwrap_or_default()
    };

    view! {
        <div class= "relative">
            <div hidden=move || { hide_btns.get() || curr() == max() }>
                <button
                    class=concatcp!(CHG_BTN, " border-t-2 rounded-t top-0 -translate-y-8")
                    on:click=move |_| change_stack(id, is_inventory, 1)
                >
                    <div class= "svg w-6" inner_html=svg::PLUS />
                </button>
            </div>
            <button
                class=move || format!("flex flex-col justify-center text-center text-sky-500 w-10 h-full px-1 {}", active_css())
                on:click=move |_| { Revealer::open(btn_id, id) }
            >
                <span class= "border-b border-sky-500 w-full"> { curr } </span>
                <span class= "w-full"> { max } </span>
            </button>
            <div hidden=move|| { hide_btns.get() || curr() < 2}>
                <button
                    class=concatcp!(CHG_BTN, " border-b-2 rounded-b bottom-0 translate-y-8")
                    on:click=move |_| change_stack(id, is_inventory, -1)
                >
                    <div class= "svg w-6" inner_html=svg::MINUS />
                </button>
            </div>
        </div>
    }
}

/// Changes the stack of an item in the quick access slot.
fn change_stack(id: usize, is_inventory: bool, by: i16) {
    PC::update(|pc| {
        let maybe_item = if is_inventory {
            pc.inventory.get_mut(id)
        } else {
            pc.quick_access[id].as_mut()
        };
        if let Some(item) = maybe_item {
            if let Some((curr, _)) = item.stacks {
                let new_curr = u8::try_from(curr as i16 + by).unwrap();
                item.stacks = item.stacks.map(|(_, max)| (new_curr, max));
            }
        }
    })
}
