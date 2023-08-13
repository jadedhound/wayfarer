use const_format::concatcp;
use leptos::*;

use crate::pc::PC;
use crate::svg;
use crate::utils::rw_context;
use crate::views::revealer::Revealer;

#[component]
pub(super) fn StackBtnInvScout(cx: Scope, id: usize) -> impl IntoView {
    let stacks = create_memo(cx, move |_| {
        rw_context::<PC>(cx).with(|pc| pc.inventory.get(&id).and_then(|item| item.stacks))
    });
    move || {
        stacks.with(|maybe_stacks| {
            maybe_stacks
                .map(|(curr, max)| view! { cx, 
                    <StackBtn btn_id='s' id curr max on_change=move |id, by| { change_inv_stack(cx, id, by) } /> 
                })
                .into_view(cx)
        })
    }
}

#[component]
pub(super) fn StackBtnQuickScout(cx: Scope, id: usize) -> impl IntoView {
    let stacks = create_memo(cx, move |_| {
        rw_context::<PC>(cx).with(|pc| pc.quick_access[id].as_ref().and_then(|item| item.stacks))
    });
    move || {
        stacks.with(|maybe_stacks| {
            maybe_stacks
                .map(|(curr, max)| view! { cx, 
                    <StackBtn btn_id='s' id curr max on_change=move |id, by| { change_quick_stack(cx, id, by) } /> 
                })
                .into_view(cx)
        })
    }
}

#[component]
fn StackBtn<F>(cx: Scope, btn_id: char, id: usize, curr: u8, max: u8, on_change: F) -> impl IntoView
where
    F: Fn(usize, i16) + 'static + Copy,
{
    const CHG_BTN: &str =
        "bg-sky-900 rounded shadow shadow-sky-500 absolute flex-centered w-10 h-10 z-50";
    let hidden = create_memo(cx, move |_| !Revealer::state(cx, btn_id, &id));

    view! {
        cx,
        <div class= "relative">
            <div hidden=move || { hidden.get() || curr == max }>
                <button
                    class=concatcp!(CHG_BTN, " top-0 -translate-y-12")
                    on:click=move |_| on_change(id, 1)
                >
                    <div class= "svg w-6" inner_html=svg::PLUS />
                </button>
            </div>
            <button
                class= "flex flex-col justify-center text-center bg-sky-900 rounded-r w-10 h-full"
                on:click=move |_| { Revealer::open(cx, 's', &id) }
            >
                <span class= "border-b w-full"> { curr } </span>
                <span class= "w-full"> { max } </span>
            </button>
            <div hidden=move|| { hidden.get() || curr < 2}>
                <button
                    class=concatcp!(CHG_BTN, " bottom-0 translate-y-12")
                    on:click=move |_| on_change(id, -1)
                >
                    <div class= "svg w-6" inner_html=svg::MINUS />
                </button>
            </div>
        </div>
    }
}

/// Changes the stack of an item in the inventory.
fn change_inv_stack(cx: Scope, id: usize, by: i16) {
    let pc = rw_context::<PC>(cx);
    pc.update(|pc| {
        let item = pc.inventory.get_mut(&id).unwrap();
        if let Some((curr, _)) = item.stacks {
            let new_curr = u8::try_from(curr as i16 + by).unwrap();
            item.stacks = item.stacks.map(|(_, max)| (new_curr, max));
        }
    })
}

/// Changes the stack of an item in the quick access slot.
fn change_quick_stack(cx: Scope, id: usize, by: i16) {
    let pc = rw_context::<PC>(cx);
    pc.update(|pc| {
        if let Some(item) = pc.quick_access[id].as_mut() {
            if let Some((curr, _)) = item.stacks {
                let new_curr = u8::try_from(curr as i16 + by).unwrap();
                item.stacks = item.stacks.map(|(_, max)| (new_curr, max));
            }
        }
    })
}
