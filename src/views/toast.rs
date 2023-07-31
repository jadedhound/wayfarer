use std::fmt::Display;

use leptos::*;

use crate::svg;
use crate::utils::rw_context;

pub struct Toast {
    hidden: bool,
    msg: String,
}

impl Toast {
    pub fn new() -> Self {
        Self {
            hidden: true,
            msg: String::new(),
        }
    }
    pub fn show<S: Display>(cx: Scope, msg: S) {
        let state = rw_context::<Toast>(cx);
        state.update(|state| {
            state.hidden = false;
            state.msg = msg.to_string();
        })
    }
}

#[component]
pub fn ToastNotif(cx: Scope) -> impl IntoView {
    let state = rw_context::<Toast>(cx);

    view! {
        cx,
        <div class= "fixed" hidden=move || state.with(|s| s.hidden)>
            <div class= "z-10 fixed bottom-2 inset-x-0 w-full p-2 flex-centered">
                <div
                    class= "animate-popin shadow shadow-blue-600 bg-blue-800 rounded flex-centered w-fit py-1 px-2 gap-2 cursor-pointer"
                    on:click=move |_| { state.update(|s| s.hidden = true) }
                >
                    <div class= "w-6 svg" inner_html=svg::INFO />
                    <div class= "flex-1">
                        { move || state.with(|s| s.msg.clone()) }
                    </div>
                </div>
            </div>
        </div>
    }
}
