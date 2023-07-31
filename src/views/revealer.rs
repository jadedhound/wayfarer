use leptos::*;

use crate::utils::rw_context;

pub struct Revealer(pub Option<u32>);

impl Revealer {
    pub fn state(cx: Scope, id: u32) -> bool {
        rw_context::<Revealer>(cx).with(|x| x.0.is_some_and(|x| x == id))
    }

    pub fn open(cx: Scope, id: u32) {
        rw_context::<Revealer>(cx).update(|state| state.0 = Some(id))
    }

    pub fn dismiss(cx: Scope) {
        rw_context::<Revealer>(cx).update(|state| state.0 = None)
    }
}

#[component]
pub fn RevealerScreen(cx: Scope) -> impl IntoView {
    let hidden = move || rw_context::<Revealer>(cx).with(|state| state.0.is_none());

    view! {
        cx,
        // Click anywhere else on the screen to dismiss
        <div
            on:click=move |_| { Revealer::dismiss(cx) }
            class= "fixed z-50 inset-0 cursor-pointer"
            hidden=hidden
        />
    }
}
