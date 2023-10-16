use std::time::Duration;

use gloo::timers::future::sleep;
use leptos::*;

use crate::icons;
use crate::items::{search, Item, ItemRef};
use crate::pc::PC;
use crate::utils::{expect_rw, some_if, RwProvided};

#[derive(Default)]
struct SearchState {
    query: String,
    is_active: bool,
    found_item: Option<Item>,
}

pub(super) fn search_view() -> impl IntoView {
    let state = create_rw_signal(SearchState::default());
    provide_context(state);
    let has_item = create_read_slice(state, |state| state.found_item.is_some());

    let item_or_textbox = move || {
        if has_item.get() {
            item_view().into_view()
        } else {
            input().into_view()
        }
    };
    let hide_results = move || state.with(|x| !x.is_active);

    view! {
        <div class= "flex flex-col gap-2">
            { item_or_textbox }
            <div hidden=hide_results>
                <div class= "flex flex-col shaded-table">
                    { results_view() }
                </div>
            </div>
        </div>
    }
}

fn item_view() -> impl IntoView {
    let state = expect_rw::<SearchState>();
    let item = state.with(|x| x.found_item.clone().unwrap_or_default());
    let item_view = item.into_view();
    let add_item = move || {
        PC::update(|pc| pc.inventory.add(item.clone()));
        state.set(SearchState::default());
    };
    let del_item = move || state.set(SearchState::default());

    view! {
        <div class= "flex gap-1">
            <button
                class= "btn-no-font bg-zinc-800 p-2 w-12 grow"
                on:click=move |_| add_item()
            >
                { item_view }
            </button>
            <button
                class= "btn bg-red-800 flex-center px-2"
                on:click=move |_| del_item()
            >
                <div class= "w-4" inner_html=icons::CROSS />
            </button>
        </div>
    }
}

fn input() -> impl IntoView {
    let state = expect_rw::<SearchState>();
    // Focus loss needs to be staggered so that search results can be
    // clicked.
    let delayed_loss = move || {
        spawn_local(async move {
            sleep(Duration::from_millis(1)).await;
            // Delay means that state might be disposed of, e.g. quickly
            // navigating after clicking on input.
            state.try_update(|x| x.is_active = false);
        })
    };

    view! {
        <input
            class= "input text-center"
            on:input=move |ev| state.update(|x| x.query = event_target_value(&ev))
            prop:value=move || state.with(|x| x.query.clone())
            on:focus=move |_| state.update(|x| x.is_active = true)
            on:blur=move |_| delayed_loss()
        />
    }
}

fn results_view() -> impl IntoView {
    let state = expect_rw::<SearchState>();
    let query = create_read_slice(state, |state| {
        some_if(!state.query.is_empty()).map(|_| state.query.clone())
    });
    let empty = move || {
        view! {
            <div class= "text-center py-2"> "Search for items or loot tables..." </div>
        }
    };
    let item_found = move |item: &ItemRef| {
        state.update(|x| x.found_item = Some(Item::from(*item)));
    };
    let nothing_found = || {
        view! {
            <div class= "text-center py-2">
                "Unable to find any items..."
            </div>
        }
        .into_view()
    };
    let item_view = move |item: &'static ItemRef| {
        view! {
            <button
                class= "text-center py-2"
                on:click=move |_| item_found(item)
            >
                { item.name }
            </button>
        }
    };
    let results_v = move |query: &str| {
        let arr: Vec<_> = search(query).take(3).collect();
        if arr.is_empty() {
            nothing_found()
        } else {
            arr.into_iter().map(item_view).collect_view()
        }
    };

    move || match query.get() {
        Some(x) => results_v(&x).into_view(),
        None => empty.into_view(),
    }
}
