use std::fmt::Display;

use leptos::*;

use crate::icons;
use crate::utils::expect_rw;

pub struct Toast {
    hidden: bool,
    title: String,
    body: String,
}

impl Toast {
    pub fn show<S: Display>(title: S, msg: S) {
        let state = expect_rw::<Toast>();
        state.set(Self {
            hidden: false,
            title: title.to_string(),
            body: msg.to_string(),
        })
    }
}

impl Default for Toast {
    fn default() -> Self {
        Self {
            hidden: true,
            title: String::new(),
            body: String::new(),
        }
    }
}

pub fn toast_notif() -> impl IntoView {
    let state = expect_rw::<Toast>();
    let is_hidden = move || state.with(|s| s.hidden);
    let hide_toast = move || state.update(|s| s.hidden = true);
    let msg = move || {
        state.with(|s| {
            view! {
                <div class= "uppercase font-tight -mb-1"> { &s.title } </div>
                <div class= "capitalise"> { format!("{}.", s.body) } </div>
            }
        })
    };

    view! {
        <div class= "fixed inset-x-0 bottom-12 z-10" hidden=is_hidden>
            <div class= "z-10 relative bottom-2 inset-x-0 w-full p-2 flex-center">
                <div
                    class= "animate-popin cursor-pointer btn bg-sky-800 flex items-center gap-2 p-2"
                    on:click=move |_| hide_toast()
                >
                    <div class= "w-5" inner_html=icons::BELL />
                    <div class= "flex-1">
                        { msg }
                    </div>
                </div>
            </div>
        </div>
    }
}
