use leptos::*;

use crate::items::{self, Item};
use crate::pc::PC;
use crate::svg;
use crate::utils::{rw_context, write_context, StrPlus};
use crate::views::InvItem;

#[derive(Default)]
struct SearchState {
    choose_quality: bool,
    quality: u8,
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
            view! {
                cx,
                <QualityBtn />
                <Textbox />
            }
            .into_view(cx)
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
    let inv_item = item.clone();
    let state = rw_context::<SearchState>(cx);
    view! {
        cx,
        <button
            class= "rounded bg-sky-900 col-span-6 py-1"
            on:click=move |_| state.update(|s| {
                write_context::<PC>(cx).update(|pc| pc.inventory.push(item.clone()));
                s.found = None;
            })
        >
            <InvItem item=inv_item />
        </button>
    }
}

#[component]
fn Textbox(cx: Scope) -> impl IntoView {
    let query = rw_context::<Query>(cx);
    view! {
        cx,
        <input
            class= "rounded col-span-5 h-10 bg-zinc-900 border-2 border-sky-900 outline-none px-1"
            on:input=move |ev| query.update(|q| q.0 = event_target_value(&ev))
            prop:value=move || query.with(|q| q.0.clone())
        />
    }
}

fn quality_to_stars(q: u8) -> &'static str {
    match q {
        3 => svg::STARS_4,
        2 => svg::STARS_3,
        1 => svg::STARS_2,
        _ => svg::STARS_1,
    }
}

#[component]
fn QualityMenu(cx: Scope, quality: u8) -> impl IntoView {
    let state = rw_context::<SearchState>(cx);
    let svg = quality_to_stars(quality);
    view! {
        cx,
        <button
            class= "bg-yellow-700 rounded w-full h-full flex-centered my-1"
            on:click=move |_| state.update(|s| {
                s.quality = quality;
                s.choose_quality = false
            })
        >
            <div class= "w-6 svg" inner_html=svg />
        </button>
    }
}

#[component]
fn QualityBtn(cx: Scope) -> impl IntoView {
    let state = rw_context::<SearchState>(cx);
    let svg = move |state: &SearchState| quality_to_stars(state.quality);
    view! {
        cx,
        <div class= "relative col-span-1">
            <div class= "absolute z-10 mb-2 -translate-y-[11.5rem] w-full h-full"
                hidden=move || state.with(|s| !s.choose_quality)
            >
                <QualityMenu quality=3 />
                <QualityMenu quality=2 />
                <QualityMenu quality=1 />
                <QualityMenu quality=0 />
            </div>
            <div class= "psuedo fixed top-0 right-0 h-cover w-full"
                hidden=move || state.with(|s| !s.choose_quality)
                on:click=move |_| state.update(|s| s.choose_quality = false)
            />
            <button
                class= "rounded flex-centered w-full h-full bg-yellow-700"
                on:click=move |_| state.update(|s| s.choose_quality = !s.choose_quality)
            >
                <div class= "w-6 svg" inner_html=move || state.with(svg) />
            </button>
        </div>
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
            class= move || state.with(|s|"rounded flex-centered col-span-1".plus(colour(s)))
            on:click=move |_| state.update(|s| {
                s.found = query.with(|q| items::search(q.0.clone(), s.quality));
                s.choose_quality = false;
                query.set(Query::default())
            })
        >
            <div class= "w-6 svg" inner_html=move || state.with(svg) />
        </button>
    }
}
