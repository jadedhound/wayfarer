use std::fmt::Display;

use leptos::*;

use crate::icons;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;

#[derive(Default)]
pub struct Toast {
    is_shown: bool,
    title: String,
    body: String,
}

impl Toast {
    pub fn show<S: Display>(title: S, msg: S) {
        let state = Toast::expect();
        state.set(Self {
            is_shown: true,
            title: title.to_string(),
            body: msg.to_string(),
        })
    }
}

impl RwUtils for Toast {
    type Item = Self;
}

pub fn toast_notification() -> impl IntoView {
    let state = Toast::expect();
    let is_hidden = move || state.with(|state| !state.is_shown);
    let hide_toast = move |_| state.reset();
    let msg = move || {
        state.with(|s| {
            view! {
                <div class= "uppercase -mb-1"> { &s.title } </div>
                <div class= "capitalise font-sans"> { format!("{}.", s.body) } </div>
            }
        })
    };

    view! {
        <div class= "fixed inset-x-0 bottom-2 px-2 z-10" hidden=is_hidden>
            <button
                class= "animate-popin text-left btn bg-sky-800 flex items-center gap-2 p-2 mx-auto"
                on:click=hide_toast
            >
                <div class= "w-5" inner_html=icons::BELL />
                <div class= "flex-1">
                    { msg }
                </div>
            </button>
        </div>
    }
}
