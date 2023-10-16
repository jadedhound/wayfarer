use std::time::Duration;

use const_format::formatcp;
use leptos::*;

use crate::buffs::search::search as find_buff;
use crate::buffs::{Buff, BuffRef};
use crate::icons;
use crate::pc::PC;
use crate::utils::{expect_rw, RwProvided};

#[derive(Default)]
struct SearchState {
    query: String,
    focus: bool,
    chosen_buff: Option<Buff>,
}

pub(super) fn search() -> impl IntoView {
    let state = create_rw_signal(SearchState::default());
    provide_context(state);
    let chosen_buff = create_read_slice(state, |x| x.chosen_buff.clone());

    move || {
        if let Some(buff) = chosen_buff.get() {
            confirm_buff(buff).into_view()
        } else {
            input().into_view()
        }
    }
}

fn confirm_buff(buff: Buff) -> impl IntoView {
    let state = expect_rw::<SearchState>();
    let buff_view = buff.into_view();
    let clear_input = move || {
        state.update(|x| {
            x.query.clear();
            x.chosen_buff = None;
        })
    };
    let accept = move || {
        let buff = buff.clone();
        PC::update(|pc| pc.buffs.add(buff));
        clear_input()
    };

    view! {
        <div class= "flex gap-1">
            <button
                class= "btn-no-font bg-surface w-12 grow text-left p-2"
                on:click=move |_| accept()
            >
                { buff_view }
            </button>
            <button
                class= "btn bg-red-700 px-2"
                on:click=move |_| clear_input()
            >
                <div class= "w-4" inner_html=icons::CROSS />
            </button>
        </div>
    }
}

fn input() -> impl IntoView {
    let state = expect_rw::<SearchState>();
    let on_input = move |str| state.update(|x| x.query = str);
    let value = move || state.with(|x| x.query.clone());
    // Focus loss needs to be staggered so that search results can be
    // clicked.
    let delayed_loss = move || {
        use gloo::timers::future::sleep;

        spawn_local(async move {
            sleep(Duration::from_millis(1)).await;
            // Delay means that state might be disposed of, e.g. quickly
            // navigating after clicking on input.
            state.try_update(|x| x.focus = false);
        })
    };

    view! {
        <input
            class= "input"
            on:input=move |ev| on_input(event_target_value(&ev))
            on:focus= move |_| state.update(|x| x.focus = true)
            on:blur=move |_| delayed_loss()
            prop:value=value
        />
        { results }
    }
}

fn results() -> impl IntoView {
    let state = expect_rw::<SearchState>();
    let query = create_read_slice(state, |x| x.query.clone());
    let empty_search = move || {
        view! {
            <div class=formatcp!("py-2 italic text-center")>
                "Search for buffs..."
            </div>
        }
    };
    let no_results = move || {
        view! {
            <div class=formatcp!("py-2 italic text-center")>
                "No buffs found..."
            </div>
        }
    };
    let is_not_focused = create_read_slice(state, |x| !x.focus);
    let buff_view = move |buff: &BuffRef| {
        let buff = *buff;
        let on_click = move || state.update(|x| x.chosen_buff = Some(buff.into()));

        view! {
            <button
                class= "capitalise py-2"
                on:click=move |_| on_click()
            >
                { buff.name }
            </button>
        }
    };
    let search_result = move || {
        let query = query.get();
        if query.is_empty() {
            empty_search().into_view()
        } else {
            let buffs: Vec<&BuffRef> = find_buff(query).into_iter().take(3).collect();
            if buffs.is_empty() {
                no_results().into_view()
            } else {
                buffs.into_iter().map(buff_view).collect_view()
            }
        }
    };

    view! {
        <div hidden=is_not_focused>
            <div class= "flex flex-col gap-1 shaded-table">
                { search_result }
            </div>
        </div>
    }
}
