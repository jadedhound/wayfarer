use leptos::*;
use leptos_router::{use_location, use_navigate, use_params_map};

use crate::icons;
use crate::pc::journal::{Journal, Note};
use crate::utils::rw_utils::RwUtils;

#[derive(Clone)]
pub struct State {
    pub id: usize,
    note: Note,
}

pub fn edit_note() -> impl IntoView {
    State::provide();

    view! {
        <h4 class= "text-center"> "EDIT NOTE" </h4>
        { title }
        { body }
    }
}

fn title() -> impl IntoView {
    let state = State::expect();
    let journal = Journal::expect();
    let (name, name_set) = State::rw_slice(
        |state| state.note.name.clone(),
        |state, val| state.note.name = val,
    );
    let no_name = State::slice(|state| state.note.name.is_empty());
    let navigate_back = move || {
        let pc_id = use_location().pathname.with(|location| {
            location
                .split('/')
                .nth(2)
                .and_then(|str| str.parse::<usize>().ok())
                .unwrap()
        });
        (use_navigate())(&format!("pc/{pc_id}/journal"), Default::default())
    };
    let save_note = move |_| {
        let State { id, note } = state.get();
        journal.update(|journal| {
            if let Some(prev_note) = journal.get_mut(id) {
                *prev_note = note
            }
        });
        navigate_back()
    };

    view! {
        <div class= "flex gap-1">
            <input
                class= "input w-12 grow"
                prop:value=name
                on:input=move |ev| name_set.set(event_target_value(&ev))
            />
            <button
                class= "btn bg-green-800"
                on:click=save_note
                disabled=no_name
            >
                <div class= "w-4" inner_html=icons::CHECKMARK />
            </button>
        </div>
    }
}

fn body() -> impl IntoView {
    let (body, body_set) = State::rw_slice(
        |state| state.note.body.clone(),
        |state, val| state.note.body = val,
    );

    view! {
        <textarea
            class= "input h-12 grow"
            prop:value=body
            on:input=move |ev| body_set.set(event_target_value(&ev))
        />
    }
}

// -----------------------------------
// TRAIT IMPLS
// -----------------------------------

impl Default for State {
    fn default() -> Self {
        let id = use_params_map()
            .get()
            .get("id")
            .and_then(|id| id.parse::<usize>().ok())
            .unwrap_or_default();
        let journal = Journal::expect();
        let note = journal.with(|journal| journal.get(id).cloned().unwrap_or_default());
        Self { id, note }
    }
}
impl RwUtils for State {}
