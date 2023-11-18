use leptos::*;

use super::note_view::note_view;
use crate::icons;
use crate::pc::journal::{Journal, Note};
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;

pub fn journal() -> impl IntoView {
    let journal = Journal::expect();
    let has_notes = Journal::slice(|journal| !journal.is_empty());
    let note_ids = move || journal.with(|journal| journal.keys().collect::<Vec<usize>>());
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
    }
}

fn new_note_input() -> impl IntoView {
    let journal = Journal::expect();
    let new_note = create_rw_signal(String::new());
    let create_note = move |_| {
        let name = new_note.get();
        journal.update_discard(|journal| journal.add(Note::new(name)));
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
                class= "btn bg-green-800"
                on:click=create_note
                disabled=move || new_note.get().is_empty()
            >
                <div class= "w-5" inner_html=icons::PLUS />
            </button>
        </div>
    }
}
