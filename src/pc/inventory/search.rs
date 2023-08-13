use leptos::*;

use crate::items::search::search;
use crate::items::Item;
use crate::pc::PC;
use crate::svg;
use crate::utils::rw_context;

#[derive(Default)]
struct SearchState {
    found: Option<Item>,
}

#[derive(Default)]
struct Query(String);

#[component]
pub(super) fn Search(cx: Scope) -> impl IntoView {
    let state = create_rw_signal(cx, SearchState::default());
    let query = create_rw_signal(cx, Query(String::new()));
    provide_context(cx, query);
    provide_context(cx, state);
    let item_or_textbox = move |state: &SearchState| {
        if let Some(item) = state.found.clone() {
            view! { cx, <Item item /> }.into_view(cx)
        } else {
            view! { cx, <Textbox /> }.into_view(cx)
        }
    };

    view! {
        cx,
        <div class= "grid grid-cols-7 gap-1 mt-4">
            { move || state.with(|s| item_or_textbox(s)) }
            <SearchBtn />
        </div>
    }
}

#[component]
fn Item(cx: Scope, item: Item) -> impl IntoView {
    let state = rw_context::<SearchState>(cx);
    let inv_item = item.clone();

    view! {
        cx,
        <button
            class= "rounded bg-zinc-800 col-span-6 py-1"
            on:click=move |_| {
                rw_context::<PC>(cx).update(|pc|{
                    pc.inventory.push(item.clone())
                });
                state.update(|s| {
                    s.found = None;
                })
            }
        >
            { inv_item.into_view(cx) }
        </button>
    }
}

#[component]
fn Textbox(cx: Scope) -> impl IntoView {
    let query = rw_context::<Query>(cx);
    view! {
        cx,
        <input
            class= "rounded col-span-6 h-10 bg-zinc-800 outline-none px-1 text-center"
            on:input=move |ev| query.update(|q| q.0 = event_target_value(&ev))
            prop:value=move || query.with(|q| q.0.clone())
        />
    }
}

#[component]
fn SearchBtn(cx: Scope) -> impl IntoView {
    let state = rw_context::<SearchState>(cx);
    let query = rw_context::<Query>(cx);
    let svg = move |state: &SearchState| {
        if state.found.is_none() {
            svg::SEARCH
        } else {
            svg::CROSS
        }
    };
    let colour = move |state: &SearchState| {
        if state.found.is_none() {
            "bg-sky-900"
        } else {
            "bg-red-900"
        }
    };
    view! {
        cx,
        <button
            class= move || state.with(|s| format!("rounded flex-centered col-span-1 {}", colour(s)))
            on:click=move |_| state.update(|s| {
                s.found = query.with(|q| search(&q.0, 0.6));
                query.set(Query::default())
            })
        >
            <div class= "w-6 svg" inner_html=move || state.with(svg) />
        </button>
    }
}
