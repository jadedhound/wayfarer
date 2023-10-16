use leptos::*;
use serde::{Deserialize, Serialize};

use super::session::PCSession;
use crate::icons;
use crate::utils::index_map::IndexMap;
use crate::utils::{expect_rw, some_if, RwProvided};
use crate::views::modal::{ModalCenter, ModalState};

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

        <h4 class= "text-center"> "Journal" </h4>
        { journal_notes }
        { new_note_input }
        { modal() }
    }
}

fn new_note_input() -> impl IntoView {
    let journals = expect_rw::<PCJournals>();
    let new_note = create_rw_signal(String::new());
    let create_note = move || {
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
                class= "btn bg-green-800 self-center w-10 text-xl"
                on:click=move |_| create_note()
                disabled=move || new_note.get().is_empty()
                inner_html=icons::PLUS
            />
        </div>
    }
}

fn note_view(id: usize, Note { name, body }: &Note) -> impl IntoView {
    let sesh = expect_rw::<PCSession>();
    let edit_note = expect_rw::<EditNote>();
    let body_hidden = PCSession::slice(move |sesh| !sesh.open_notes.contains(&id));
    let change_open = move || {
        let is_open = !body_hidden.get();
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
    let rotate_chev = move || {
        some_if(!body_hidden.get())
            .map(|_| "rotate-180")
            .unwrap_or_default()
    };

    view! {
        <div class= "flex flex-col gap-2">
            <div class= "flex gap-1">
                <button
                    class= "btn bg-purple-800 text-left px-2 h-12"
                    on:click=move |_| edit_note_modal()
                    hidden=body_hidden
                >
                    <div class= "w-6 icons" inner_html=icons::QUILL />
                </button>
                <button
                    class= "btn bg-surface text-left px-2 h-12 flex-center w-full"
                    on:click=move |_| change_open()
                >
                    <div class= "w-12 grow"> { name.clone() } </div>
                    <div class=move || format!("w-4 {}", rotate_chev()) inner_html=icons::DOWN_CHEV />
                </button>
            </div>
            <div
                class= "rounded border-2 border-zinc-800"
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
            <div class= "psuedo h-10" />
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
        <ModalCenter id=0>
            <h5> "EDIT NOTE" </h5>
            <div class= "flex flex-col gap-2">
                <input
                    class= "input"
                    prop:value=move || name.get()
                    on:input=move |ev| name.set(event_target_value(&ev))
                />
                <textarea
                    class= "input h-[40vh]"
                    prop:value=move || body.get()
                    on:input=move |ev| body.set(event_target_value(&ev))
                />
                <button
                    class=move || format!("flex-center p-2 rounded {}", btn_state().1)
                    on:click=move |_| save_or_delete()
                >
                    { move || btn_state().0 }
                </button>
            </div>
        </ModalCenter>
    }
}
