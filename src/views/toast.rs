use std::fmt::Display;

use leptos::*;

use crate::svg;
use crate::utils::expect_rw;

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
    pub fn show<S: Display>(msg: S) {
        let state = expect_rw::<Toast>();
        state.update(|state| {
            state.hidden = false;
            state.msg = msg.to_string();
        })
    }
}

pub fn toast_notification() -> impl IntoView {
    let state = expect_rw::<Toast>();

    view! {
        <div class= "fixed inset-x-0 bottom-0 z-10" hidden=move || state.with(|s| s.hidden)>
            <div class= "z-10 relative bottom-2 inset-x-0 w-full p-2 flex-centered">
                <div
                    class= "animate-popin border-2 border-sky-600 bg-sky-900 rounded flex-centered w-fit py-1 px-2 gap-2 cursor-pointer"
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
