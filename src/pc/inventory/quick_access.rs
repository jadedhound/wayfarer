use const_format::concatcp;
use leptos::*;

use crate::items::Item;
use crate::pc::inventory::stack_btn::StackBtnQuickScout;
use crate::pc::PC;
use crate::utils::rw_context;
use crate::views::modal::{ModalCentered, ModalState};

struct ChangeSlot(usize);

#[component]
pub(super) fn QuickAccess(cx: Scope) -> impl IntoView {
    provide_context(cx, create_rw_signal(cx, ChangeSlot(0)));
    let slots = move || {
        rw_context::<PC>(cx).with(|pc| {
            pc.quick_access
                .iter()
                .enumerate()
                .map(|(id, item)| {
                    view! { cx, <QuickSlot id item /> }
                })
                .collect_view(cx)
        })
    };

    view! {
        cx,
        <div class= "flex flex-col gap-2">
            { slots }
        </div>
        <ChangeSlotModal />
    }
}

#[component]
fn QuickSlot<'a>(cx: Scope, id: usize, item: &'a Option<Item>) -> impl IntoView {
    let has_stacks = item.as_ref().is_some_and(|x| x.stacks.is_some());
    let item = match item {
        Some(item) => item.into_view(cx),
        None => view! { cx,
            <div class= "h-12 flex-centered"> "Empty" </div>
        }
        .into_view(cx),
    };
    let class = if has_stacks {
        "rounded-l bg-zinc-800 gap-2 w-full"
    } else {
        "rounded bg-zinc-800 gap-2 w-full"
    };
    let stacks = if has_stacks {
        view! { cx, <StackBtnQuickScout id /> }.into_view(cx)
    } else {
        ().into_view(cx)
    };

    view! {
        cx,
        <div class= "flex">
            <button
                class=class
                on:click= move |_| {
                    rw_context::<ChangeSlot>(cx).update(|change| change.0 = id);
                    ModalState::open(cx, 0);
                }
            >
                { item }
            </button>
            { stacks }
        </div>
    }
}

#[component]
fn ChangeSlotModal(cx: Scope) -> impl IntoView {
    const BTN_CSS: &str = "rounded bg-zinc-700 py-2";
    let pc = rw_context::<PC>(cx);
    let set_slot = move |id: Option<usize>| {
        let slot = rw_context::<ChangeSlot>(cx).with(|x| x.0);
        pc.update(|pc| {
            if let Some(id) = id {
                // Assume item id is always valid.
                let item = pc.inventory.remove(&id).unwrap();
                let prev_item = pc.quick_access[slot].replace(item);
                if let Some(item) = prev_item {
                    pc.inventory.push(item);
                }
            } else if let Some(item) = pc.quick_access[slot].take() {
                pc.inventory.push(item);
            }
        });
        ModalState::dismiss(cx)
    };
    let item_view = move || {
        pc.with(|pc| {
            pc.inventory
                .iter()
                .map(|(i, item)| {
                    let i = *i;
                    view! {
                        cx,
                        <button
                            class=BTN_CSS
                            on:click=move |_| set_slot(Some(i))
                        >
                            { item.into_view(cx) }
                        </button>
                    }
                })
                .collect_view(cx)
        })
    };

    view! {
        cx,
        <ModalCentered title=|| "QUICK ACCESS" id=0>
            <div class= "flex flex-col gap-2">
                { item_view }
                <button
                    class=concatcp!(BTN_CSS, " px-2 text-start")
                    on:click=move |_| set_slot(None)
                >
                    "None"
                </button>
            </div>
        </ModalCentered>
    }
}
