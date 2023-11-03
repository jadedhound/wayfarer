use leptos::*;
use leptos_router::A;

use crate::icons;
use crate::pc::journal::Journal;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;
use crate::views::delete_confirm::DeleteModal;

pub fn note_view(id: usize) -> impl IntoView {
    let body_hidden = PC::slice(move |pc| !pc.open_notes.contains(&id));
    DeleteModal::set_effect(move |id| Journal::expect().update_discard(|journ| journ.0.remove(id)));

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
    let pc = PC::expect();
    let name = Journal::slice(move |journ| journ.0.get(id).map(|x| x.name.clone()));
    let body_shown = PC::slice(move |pc| pc.open_notes.contains(&id));
    let body_hidden = move || !body_shown.get();
    let change_open = move |_| {
        pc.update(|pc| {
            if body_shown.get() {
                if let Some(i) = pc.open_notes.iter().position(|x| x == &id) {
                    pc.open_notes.remove(i);
                }
            } else {
                pc.open_notes.push(id)
            }
        })
    };

    view! {
        <div class= "btn bg-surface text-left flex-center gap-2 w-full relative">
            <div hidden=body_hidden>
                <A href=format!("../edit_note/{id}")>
                    <div class= "w-5" inner_html=icons::QUILL />
                </A>
            </div>
            <button
                class= "w-12 grow text-left"
                on:click=change_open
            >
                { name }
            </button>
            <button
                on:click=move |_| DeleteModal::show(id)
                hidden=body_hidden
            >
                <div class= "w-5 fill-red-500" inner_html=icons::TRASH />
            </button>
        </div>
    }
}

fn help_or_body(id: usize) -> impl IntoView {
    let body = Journal::slice(move |journ| journ.0.get(id).map(|x| x.body.clone()));

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
