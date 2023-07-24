use std::time::Duration;

use gloo::timers::future::sleep;
use leptos::*;

use crate::svg;

pub struct Toast {
    hidden: bool,
    msg: String,
}

impl Toast {
    pub fn provide(cx: Scope) -> RwSignal<Self> {
        let state = create_rw_signal(
            cx,
            Toast {
                hidden: true,
                msg: "".into(),
            },
        );
        create_effect(cx, move |_| {
            let hidden = state.with(|s| s.hidden);
            if !hidden {
                spawn_local(async move {
                    sleep(Duration::from_secs(2)).await;
                    state.update(|s| s.hidden = true);
                });
            }
        });
        state
    }
    pub fn show<S>(&mut self, msg: S)
    where
        S: std::fmt::Display,
    {
        self.hidden = false;
        self.msg = msg.to_string();
    }
}

#[component]
pub fn ToastNotif(cx: Scope, state: RwSignal<Toast>) -> impl IntoView {
    view! {
        cx,
        <div hidden=move || state.with(|s| s.hidden)>
            <div class= "z-10 fixed bottom-2 inset-x-0 w-full p-2 flex-centered">
                <div class= "shadow-md shadow-zinc-950 bg-blue-800 rounded flex-centered w-fit py-1 px-2 gap-2">
                    <div class= "w-6 svg" inner_html=svg::INFO />
                    <div class= "flex-1">
                        { move || state.with(|s| s.msg.clone()) }
                    </div>
                </div>
            </div>
        </div>
    }
}
