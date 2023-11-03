use std::time::Duration;

use leptos::*;

use crate::buffs::{self, Buff, BuffRef};
use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::utils::{search as utils_search, RwSignalEnhance};

#[derive(Default)]
struct State {
    query: String,
    focus: bool,
    chosen_buff: Option<&'static BuffRef>,
}

impl RwUtils for State {}

pub fn search() -> impl IntoView {
    State::provide();
    let chosen_buff = State::slice(|x| x.chosen_buff);

    move || {
        if let Some(buff) = chosen_buff.get() {
            confirm_buff(buff).into_view()
        } else {
            input().into_view()
        }
    }
}

fn confirm_buff(buff_ref: &'static BuffRef) -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let buff_view = Buff::from(*buff_ref).into_view();
    let accept = move || {
        pc.update(|pc| pc.buffs.add(Buff::from(*buff_ref)));
        state.reset();
    };

    view! {
        <div class= "flex gap-1">
            <button
                class= "btn !font-[inherit] bg-surface w-12 grow text-left"
                on:click=move |_| accept()
            >
                { buff_view }
            </button>
            <button
                class= "btn bg-red-700"
                on:click=move |_| state.reset()
            >
                <div class= "w-4" inner_html=icons::CROSS />
            </button>
        </div>
    }
}

fn input() -> impl IntoView {
    let state = State::expect();
    let (query, query_set) = State::rw_slice(
        |state| state.query.clone(),
        |state, value| state.query = value,
    );
    // Focus loss needs to be staggered so that search results can be
    // clicked.
    let delayed_focus_loss = move |_| {
        use gloo::timers::future::sleep;
        spawn_local(async move {
            sleep(Duration::from_millis(1)).await;
            state.try_update(|x| x.focus = false);
        })
    };

    view! {
        <div class= "relative">
            <div class= "absolute inset-y-0 left-2 flex-center" >
                <div class= "w-6 stroke-sky-500" inner_html=icons::MAGNIFYING_GLASS />
            </div>
            <input
                class= "input text-center w-full !px-10"
                placeholder= "Search for buffs..."
                on:focus= move |_| state.update(|x| x.focus = true)
                on:blur=delayed_focus_loss
                on:input=move |ev| query_set.set(event_target_value(&ev))
                prop:value=query
            />
        </div>
        { results }
    }
}

fn results() -> impl IntoView {
    let query = leptos_use::signal_debounced(State::slice(|state| state.query.clone()), 500.0);
    let is_not_focused = State::slice(|x| !x.focus);
    let find_buff = move |query: String| utils_search::search(&buffs::ALL, |buff| buff.name, query);
    let search_result = move || {
        let query = query.get();
        if query.is_empty() {
            view! {
                <div class= "py-2 italic text-center">
                    "Search for buffs..."
                </div>
            }
            .into_view()
        } else {
            let buffs: Vec<&BuffRef> = find_buff(query).into_iter().take(3).collect();
            if buffs.is_empty() {
                view! {
                    <div class= "py-2 italic text-center">
                        "No buffs found..."
                    </div>
                }
                .into_view()
            } else {
                buffs.into_iter().map(buffref_button).collect_view()
            }
        }
    };

    view! {
        <div class= "flex flex-col gap-1 shaded-table" hidden=is_not_focused>
            { search_result }
        </div>
    }
}

fn buffref_button(buff: &'static BuffRef) -> impl IntoView {
    let state = State::expect();
    let add_buff = move |_| state.update(|x| x.chosen_buff = Some(buff));

    view! {
        <button
            class= "capitalise py-2"
            on:click=add_buff
        >
            { buff.name }
        </button>
    }
}
