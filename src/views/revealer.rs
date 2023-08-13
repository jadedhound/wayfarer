use leptos::*;

use crate::utils::rw_context;

pub struct Revealer(pub Option<(char, usize)>);

impl Revealer {
    /// Returns true if the current state matches the `origin` and `id` given.
    pub fn state(cx: Scope, origin: char, id: &usize) -> bool {
        rw_context::<Revealer>(cx).with(|x| x.0.is_some_and(|(o, i)| o == origin && &i == id))
    }

    pub fn open(cx: Scope, origin: char, id: &usize) {
        rw_context::<Revealer>(cx).update(|state| state.0 = Some((origin, *id)))
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
            class= "fixed z-30 inset-0 cursor-pointer"
            hidden=hidden
        />
    }
}
