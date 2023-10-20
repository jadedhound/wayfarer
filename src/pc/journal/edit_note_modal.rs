use leptos::*;

use crate::icons;
use crate::pc::journal::{Note, PCJournals};
use crate::utils::expect_rw;
use crate::utils::rw_utils::RwUtils;
use crate::views::modal::{ModalCustom, ModalState};

#[derive(Default, Clone)]
pub struct State {
    pub id: Option<usize>,
    name: String,
    body: String,
}

impl RwUtils for State {
    type Item = State;
}

pub fn edit_note_modal() -> impl IntoView {
    let journals = expect_rw::<PCJournals>();
    // State is provided early in journal::journal.
    let state = State::expect();
    let id = State::slice(|state| state.id);
    // Whenever id changes, update the title and body to match.
    create_effect(move |_| {
        if let Some(id) = id.get() {
            let note = journals.with_untracked(|j| j.0.get(id).cloned());
            if let Some(Note { name, body }) = note {
                state.update(|state| {
                    state.name = name;
                    state.body = body;
                })
            }
        }
    });

    view! {
        <ModalCustom id=10>
            <div class= "animate-popin p-2 bg-zinc-900 flex flex-col gap-2 h-full">
                <h4 class= "text-center"> "EDIT NOTE" </h4>
                { title }
                { body }
            </div>
        </ModalCustom>
    }
}

fn title() -> impl IntoView {
    let state = State::expect();
    let journals = expect_rw::<PCJournals>();
    let (name, name_set) =
        State::rw_slice(|state| state.name.clone(), |state, val| state.name = val);
    let save_note = move |_| {
        ModalState::hide();
        let State { id, name, body } = state.get();
        journals.update(|j| {
            if let Some(note) = j.0.get_mut(id.unwrap()) {
                note.name = name;
                note.body = body;
            }
        })
    };

    view! {
        <div class= "flex gap-1">
            <button
                class= "btn bg-red-800 px-2"
                on:click=move |_| ModalState::hide()
            >
                <div class= "w-4" inner_html=icons::CROSS />
            </button>
            <input
                class= "input w-12 grow"
                prop:value=name
                on:input=move |ev| name_set.set(event_target_value(&ev))
            />
            <button
                class= "btn bg-green-800 px-2"
                on:click=save_note
            >
                <div class= "w-4" inner_html=icons::CHECKMARK />
            </button>
        </div>
    }
}

fn body() -> impl IntoView {
    let (body, body_set) =
        State::rw_slice(|state| state.body.clone(), |state, val| state.body = val);

    view! {
        <textarea
            class= "input h-12 grow"
            prop:value=body
            on:input=move |ev| body_set.set(event_target_value(&ev))
        />
    }
}
