use leptos::*;

use crate::icons;
use crate::pc::journal::{edit_note_modal, PCJournals};
use crate::pc::session::Session;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;
use crate::views::delete_confirm::DeleteModal;
use crate::views::modal::ModalState;

pub fn note_view(id: usize) -> impl IntoView {
    let body_hidden = Session::slice(move |sesh| !sesh.open_notes.contains(&id));
    DeleteModal::set_effect(move |id| {
        PCJournals::expect().update_discard(|journ| journ.0.remove(id))
    });
    view! {
        { title(id) }
        <div
            class= "rounded border-2 border-zinc-800"
            hidden=body_hidden
        >
            { help_or_body(id) }
        </div>
    }
}

fn title(id: usize) -> impl IntoView {
    let edit_modal_state = edit_note_modal::State::expect();
    let sesh = Session::expect();
    let name = PCJournals::slice(move |journ| journ.0.get(id).map(|x| x.name.clone()));
    let body_shown = Session::slice(move |sesh| sesh.open_notes.contains(&id));
    let body_hidden = move || !body_shown.get();
    let change_open = move |_| {
        sesh.update(|sesh| {
            if body_shown.get() {
                if let Some(i) = sesh.open_notes.iter().position(|x| x == &id) {
                    sesh.open_notes.remove(i);
                }
            } else {
                sesh.open_notes.push(id)
            }
        })
    };
    let open_modal = move |_| {
        edit_modal_state.update(|state| state.id = Some(id));
        ModalState::show(10);
    };

    view! {
        <div
            class= "btn bg-surface text-left flex-center w-full relative"
        >
            <button
                class= "pl-2"
                on:click=open_modal
                hidden=body_hidden
            >
                <div class= "w-5" inner_html=icons::QUILL />
            </button>
            <button
                class= "p-2 w-12 grow text-left"
                on:click=change_open
            >
                { name }
            </button>
            <button
                class= "pr-2"
                on:click=move |_| DeleteModal::show(id)
                hidden=body_hidden
            >
                <div class= "w-5 fill-red-500" inner_html=icons::TRASH />
            </button>
        </div>
    }
}

fn help_or_body(id: usize) -> impl IntoView {
    let body = PCJournals::slice(move |journ| journ.0.get(id).map(|x| x.body.clone()));

    move || {
        body.get().map(|body| {
            if body.is_empty() {
                view! {
                    <div class= "py-2 text-center italic">
                        "Tap the quill to pen a note."
                    </div>
                }
            } else {
                view! {
                    <div class= "p-2 whitespace-pre-wrap">
                        { body }
                    </div>
                }
            }
        })
    }
}
