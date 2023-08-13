use const_format::concatcp;
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::utils::index_map::IndexMap;
use crate::utils::rw_context;
use crate::{css, svg};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCJournals(IndexMap<Note>);

#[derive(Serialize, Deserialize, Clone)]
struct Note {
    name: String,
    body: String,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            name: "New Note".into(),
            body: String::new(),
        }
    }
}

pub fn journal(cx: Scope) -> impl IntoView {
    let journals = rw_context::<PCJournals>(cx);
    let journal_notes =
        move || journals.with(|j| j.0.values().map(|x| note_view(cx, x)).collect_view(cx));

    view! {
        cx,
        <div class= "flex flex-col px-2">
            <h4 class= "text-center"> "NOTES" </h4>
            <div class= "">
                { journal_notes }
            </div>
            <button
                class=concatcp!(css::BTN, " font-sans flex-centered h-12 gap-x-2 mt-6")
                on:click=move |_| journals.update(|j| j.0.push(Note::default()))
            >
                <div class= "svg w-6" inner_html=svg::PLUS />
                <div> "NEW NOTE" </div>
            </button>
        </div>
    }
}

fn note_view(cx: Scope, Note { name, body }: &Note) -> impl IntoView {
    view! { cx,
        <div class= "">
            { name.clone() }
        </div>
        <div class= "">
            { body.clone() }
        </div>
    }
}
