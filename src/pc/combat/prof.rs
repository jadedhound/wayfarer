use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

#[derive(Default, Clone, Copy)]
struct State {
    editing: bool,
}

impl RwUtils for State {}

pub(super) fn prof_view() -> impl IntoView {
    let state = State::provide();
    move || {
        if state.get().editing {
            EditInput().into_view()
        } else {
            FormattedDisplay().into_view()
        }
    }
}

#[component]
fn FormattedDisplay() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let open_input = move || state.update(|x| x.editing = true);
    let msg = move || pc.with(|pc| format!("You are proficient in {} checks.", pc.prof));

    view! {
        <button class= "btn bg-surface flex-center" on:click=move |_| open_input()>
            <div class= "w-5" inner_html=icons::QUILL />
        </button>
        <div class= "col-span-6">
            { msg }
        </div>
    }
}

#[component]
fn EditInput() -> impl IntoView {
    let state = State::expect();
    let (prof, prof_set) = PC::rw_slice(|pc| pc.prof.clone(), |pc, value| pc.prof = value);
    let close_input = move || state.update(|x| x.editing = false);

    view! {
        <button class= "btn bg-green-800 flex-center" on:click=move |_| close_input()>
            <div class= "w-4" inner_html=icons::CHECKMARK />
        </button>
        <input
            class= "col-span-6 input"
            on:input=move |ev| prof_set.set(event_target_value(&ev))
            prop:value=prof
        />
    }
}
