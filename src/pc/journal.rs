use leptos::*;
use serde::{Deserialize, Serialize};

use self::edit_note_modal::edit_note_modal;
use self::note_view::note_view;
use crate::icons;
use crate::utils::expect_rw;
use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;

mod edit_note_modal;
mod note_view;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCJournals(IndexMap<Note>);

impl RwUtils for PCJournals {
    type Item = Self;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    name: String,
    body: String,
}

impl Note {
    fn new(name: String) -> Self {
        Self {
            name,
            body: String::new(),
        }
    }
}

#[component]
pub fn Journal() -> impl IntoView {
    edit_note_modal::State::provide();
    let journals = PCJournals::expect();
    let has_notes = PCJournals::slice(|journ| !journ.0.is_empty());
    let note_ids = move || journals.with(|journ| journ.0.keys().collect::<Vec<usize>>());
    let opt_notes = move || {
        has_notes.get().then(|| {
            view! {
                <For
                    each=note_ids
                    key=|id| *id
                    children=note_view
                />
            }
        })
    };

    view! {
        <h4 class= "text-center"> "Journal" </h4>
        { opt_notes }
        { new_note_input }
        { edit_note_modal }
    }
}

fn new_note_input() -> impl IntoView {
    let journals = expect_rw::<PCJournals>();
    let new_note = create_rw_signal(String::new());
    let create_note = move |_| {
        let name = new_note.get();
        journals.update(|j| j.0.add(Note::new(name)));
        new_note.set(String::new())
    };

    view! {
        <div class= "flex gap-1">
            <input
                class= "w-12 grow input"
                on:input=move |ev| new_note.set(event_target_value(&ev))
                prop:value=move || new_note.get()
            />
            <button
                class= "btn bg-green-800 px-1"
                on:click=create_note
                disabled=move || new_note.get().is_empty()
            >
                <div class= "w-8" inner_html=icons::PLUS />
            </button>
        </div>
    }
}
