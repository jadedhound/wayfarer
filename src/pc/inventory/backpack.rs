mod usable;

use const_format::concatcp;
use leptos::*;

use crate::items::Item;
use crate::pc::inventory::stack_btn::StackBtnInvScout;
use crate::pc::session::PCSession;
use crate::pc::{MAX_CAPACITY, PC};
use crate::svg;
use crate::utils::rw_context;
use crate::views::revealer::Revealer;

#[derive(Clone, Copy)]
struct DeleteState(Option<usize>);

#[component]
pub fn Backpack(cx: Scope) -> impl IntoView {
    let delete = create_rw_signal(cx, DeleteState(None));
    provide_context(cx, delete);

    view! {
        cx,
        <BackpackListView />
        <div
            class= "psuedo fixed h-cover w-full z-10 top-0 right-0"
            hidden=move || delete.with(|d| d.0.is_none())
            on:click=move |_| delete.update(|d| d.0 = None)
        />
    }
}

#[component]
fn BackpackListView(cx: Scope) -> impl IntoView {
    let id_list = move || {
        rw_context::<PC>(cx).with(|pc| {
            pc.inventory
                .iter()
                .map(|(id, item)| (*id, item.clone()))
                .collect::<Vec<(usize, Item)>>()
        })
    };
    view! { cx,
        <div class= "flex flex-col gap-y-2 mt-4">
            <For
                each=id_list
                key=|(id, _)| *id
                view=move |cx, (id, item)| { view!{ cx, <BackpackItem id item /> } }
            />
        </div>
    }
}

/// Renders the item with the `id` given.
#[component]
fn BackpackItem(cx: Scope, id: usize, item: Item) -> impl IntoView {
    const NORMAL: &str = "bg-zinc-800 w-full ";
    const NO_STACKS: &str = "rounded-r";
    let delete = move || {
        rw_context::<PC>(cx).update(|pc| {
            pc.inventory.remove(&id);
            Revealer::dismiss(cx);
        })
    };

    let class = match item.stacks {
        Some(_) => NORMAL,
        None => concatcp!(NORMAL, NO_STACKS),
    };

    view! {
        cx,
        <div class= "relative">
            <button
                class= "flex items-stretch w-full"
                on:contextmenu=move |_| {
                    Revealer::open(cx, 'd', &id);
                }
            >
                <WeightView id />
                <div class=class>
                    { item.into_view(cx) }
                </div>
                <StackBtnInvScout id />
            </button>
            <button
                class= "absolute inset-0 bg-red-800 z-50 rounded w-full"
                on:click=move |_| delete()
                hidden=move || !Revealer::state(cx, 'd', &id)
            >
                <div class= "flex-centered gap-x-2">
                    <div class= "svg w-6" inner_html=svg::TRASH />
                    <div> { format!("Delete {}?", item.name) } </div>
                </div>
            </button>
        </div>
    }
}

#[component]
fn WeightView(cx: Scope, id: usize) -> impl IntoView {
    let inv_slots = create_memo(cx, move |_| {
        rw_context::<PCSession>(cx).with(|sesh| sesh.inv_slots.get(&id).cloned().unwrap_or((0, 0)))
    });
    let text = move || {
        let (start, end) = inv_slots.get();
        if start != end {
            view! { cx,
                <div class= "flex flex-col text-center">
                    <span> {start} </span>
                    <span> {end} </span>
                </div>
            }
            .into_view(cx)
        } else {
            start.into_view(cx)
        }
    };
    let css = move || {
        let (start, end) = inv_slots.get();
        if start > MAX_CAPACITY || end > MAX_CAPACITY {
            ("bg-red-800", "")
        } else {
            ("bg-zinc-800", "border-r-2 border-sky-700")
        }
    };
    view! {
        cx,
        <div class=move || format!("rounded-l w-10 pr-0.5 -mr-0.5 flex items-center {}", css().0)>
            <div class=move || format!("{} w-full h-10 flex-centered", css().1)> { text } </div>
        </div>
    }
}
