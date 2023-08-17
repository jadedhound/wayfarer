use leptos::*;
use serde::{Deserialize, Serialize};

use crate::svg;
use crate::utils::index_map::IndexMap;
use crate::utils::{expect_rw, some_if};
use crate::views::modal::{ModalCentered, ModalState};

use super::session::PCSession;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCJournals(IndexMap<Note>);

#[derive(Serialize, Deserialize, Clone)]
struct Note {
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

struct EditNote(usize);

pub fn journal() -> impl IntoView {
    provide_context(create_rw_signal(EditNote(0)));
    let journals = expect_rw::<PCJournals>();
    let journal_notes = move || {
        journals.with(|j| {
            some_if(!j.0.is_empty()).map(|_| {
                let v = j.0.iter().map(|(id, x)| note_view(id, x)).collect_view();
                view! {
                    <div class= "flex flex-col gap-2">
                        { v }
                    </div>
                }
            })
        })
    };

    view! {

        <div class= "flex flex-col px-2 gap-4">
            <h5 class= "text-center"> "NOTES" </h5>
            { journal_notes }
            { new_note_input }
        </div>
        { modal() }
    }
}

fn new_note_input() -> impl IntoView {
    let journals = expect_rw::<PCJournals>();
    let new_note = create_rw_signal(String::new());

    view! {
        <div class= "flex">
            <input
                class= "w-12 grow rounded-l bg-inherit outline-none border-y-2 border-l-2 border-purple-800 px-2"
                on:input=move |ev| new_note.set(event_target_value(&ev))
                prop:value=move || new_note.get()
            />
            <button
                class= "bg-purple-800 disabled:border-purple-800 btn-rounded-r flex-centered w-12"
                on:click=move |_| {
                    let name = new_note.get();
                    journals.update(|j| j.0.add(Note::new(name)));
                    new_note.set(String::new())
                }
                disabled=move || new_note.get().is_empty()
            >
                <div class= "svg w-6" inner_html=svg::PLUS />
            </button>
        </div>
    }
}

fn note_view(id: usize, Note { name, body }: &Note) -> impl IntoView {
    let sesh = expect_rw::<PCSession>();
    let edit_note = expect_rw::<EditNote>();
    let body_hidden = move || sesh.with(|sesh| !sesh.open_notes.contains(&id));
    let chev = move || {
        if body_hidden() {
            svg::DOWN_CHEV
        } else {
            svg::UP_CHEV
        }
    };
    let change_open = move || {
        let is_open = !body_hidden();
        sesh.update(|sesh| {
            if is_open {
                if let Some(i) = sesh.open_notes.iter().position(|x| x == &id) {
                    sesh.open_notes.remove(i);
                }
            } else {
                sesh.open_notes.push(id)
            }
        })
    };
    let edit_note_modal = move || {
        edit_note.update(|x| x.0 = id);
        ModalState::open(0);
    };

    view! {
        <div class= "flex flex-col gap-2">
            <div class= "flex gap-x-2">
                <button
                    class= "btn-zinc text-left px-2 h-12"
                    on:click=move |_| edit_note_modal()
                    hidden=body_hidden
                >
                    <div class= "w-6 svg" inner_html=svg::INKWELL />
                </button>
                <button
                    class= "btn-zinc text-left px-2 h-12 flex-centered w-full"
                    on:click=move |_| change_open()
                >
                    <div class= "w-12 grow"> { name.clone() } </div>
                    <div class= "">
                        <div class= "w-6 svg" inner_html=chev />
                    </div>
                </button>
            </div>
            <div
                class= "rounded border-2 border-zinc-700"
                hidden=body_hidden
            >
                { body_view( body) }
            </div>
        </div>
    }
}

fn body_view(body: &str) -> impl IntoView {
    if body.is_empty() {
        view! {
            <div class= "text-center">
                "Click the inkwell to edit this note"
            </div>
        }
    } else {
        view! {
            <div class= "p-2 whitespace-pre-wrap">
                { body.to_string() }
            </div>
        }
    }
}

fn modal() -> impl IntoView {
    let journals = expect_rw::<PCJournals>();
    let edit_note = expect_rw::<EditNote>();
    let id = move || edit_note.with(|x| x.0);
    let name = create_rw_signal(String::new());
    let body = create_rw_signal(String::new());
    // Whenever id changes, update the title and body to match.
    create_effect(move |_| {
        journals.with(|j| {
            if let Some(note) = j.0.get(id()) {
                name.set(note.name.clone());
                body.set(note.body.clone());
            }
        });
    });
    let no_name = create_memo(move |_| name.with(|x| x.is_empty()));
    let save_or_delete = move || {
        let id = id();
        journals.update(|j| {
            if no_name.get() {
                j.0.remove(id);
            } else {
                let note = j.0.get_mut(id).unwrap();
                note.name = name.get();
                note.body = body.get();
            }
        });
        ModalState::dismiss()
    };
    let btn_state = move || {
        if no_name.get() {
            ("Delete", "bg-red-800")
        } else {
            ("Save", "bg-sky-800")
        }
    };

    view! {
        <ModalCentered title=|| "Edit Note" id=0>
            <div class= "flex flex-col gap-2">
                <input
                    class= "input"
                    prop:value=move || name.get()
                    on:input=move |ev| name.set(event_target_value(&ev))
                />
                <textarea
                    class= "input h-[50vh]"
                    prop:value=move || body.get()
                    on:input=move |ev| body.set(event_target_value(&ev))
                />
                <button
                    class=move || format!("flex-centered p-2 rounded {}", btn_state().1)
                    on:click=move |_| save_or_delete()
                >
                    { move || btn_state().0 }
                </button>
            </div>
        </ModalCentered>
    }
}
