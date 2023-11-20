use std::time::Duration;

use gloo::timers::future::sleep;
use leptos::*;

use crate::items::{Item, ItemRef};
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::utils::{expect_rw, search as utils_search, ArrayEnhance, RwSignalEnhance};
use crate::{icons, items};

#[derive(Default)]
struct State {
    query: String,
    is_active: bool,
    found_item: Option<Item>,
}

impl RwUtils for State {}

pub(super) fn search_view() -> impl IntoView {
    let state = create_rw_signal(State::default());
    provide_context(state);
    let has_item = create_read_slice(state, |state| state.found_item.is_some());

    let item_or_textbox = move || {
        if has_item.get() {
            item_view().into_view()
        } else {
            input().into_view()
        }
    };
    let hide_results = move || state.with(|state| !state.is_active || state.query.is_empty());

    view! {
        <div class= "flex flex-col gap-2">
            { item_or_textbox }
            <div hidden=hide_results>
                <div class= "flex flex-col shaded-table">
                    { results_view }
                </div>
            </div>
        </div>
    }
}

fn item_view() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let item = state.with(|x| x.found_item.clone().unwrap_or_default());
    let item_view = item.into_view();
    let add_item = move || {
        pc.update_discard(|pc| pc.backpack.add(item.clone()));
        state.set(State::default());
    };
    let del_item = move || state.set(State::default());

    view! {
        <div class= "flex gap-1">
            <button
                class= "btn bg-zinc-800 w-12 grow !font-[inherit]"
                on:click=move |_| add_item()
            >
                { item_view }
            </button>
            <button
                class= "btn bg-red-800 flex-center"
                on:click=move |_| del_item()
            >
                <div class= "w-4" inner_html=icons::CROSS />
            </button>
        </div>
    }
}

fn input() -> impl IntoView {
    let state = expect_rw::<State>();
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
        <div class= "relative">
            <div class= "absolute inset-y-0 left-2 flex-center" >
                <div class= "w-6 stroke-sky-500" inner_html=icons::MAGNIFYING_GLASS />
            </div>
            <input
                class= "input text-center w-full !px-10"
                placeholder= "Search for items..."
                on:input=move |ev| state.update(|x| x.query = event_target_value(&ev))
                prop:value=move || state.with(|x| x.query.clone())
                on:focus=move |_| state.update(|x| x.is_active = true)
                on:blur=move |_| delayed_loss()
            />
        </div>
    }
}

fn results_view() -> impl IntoView {
    let state = State::expect();
    let find_item = move |query: String| utils_search::search(&items::ALL, |item| item.name, query);
    let query = leptos_use::signal_debounced(
        State::slice(|state| state.query.is_not_empty().then(|| state.query.clone())),
        500.0,
    );
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
    let results_v = move |query: String| {
        let arr: Vec<_> = find_item(query).into_iter().take(3).collect();
        if arr.is_empty() {
            nothing_found()
        } else {
            arr.into_iter().map(item_view).collect_view()
        }
    };

    move || query.get().map(results_v).into_view()
}
