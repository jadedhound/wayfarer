use leptos::*;

use crate::utils::expect_rw;

pub struct Revealer(pub Option<(char, usize)>);

impl Revealer {
    /// Returns true if the current state matches the `origin` and `id` given.
    pub fn is_shown() -> bool {
        expect_rw::<Revealer>().with(|x| x.0.is_some())
    }

    /// Returns true if the current state matches the `origin` and `id` given.
    pub fn state(origin: char, id: usize) -> bool {
        expect_rw::<Revealer>().with(|x| x.0.is_some_and(|(o, i)| o == origin && i == id))
    }

    /// Returns true if the current state DOES NOT match the `origin` and `id` given.
    pub fn hidden(origin: char, id: usize) -> bool {
        !Self::state(origin, id)
    }

    pub fn show(origin: char, id: usize) {
        expect_rw::<Revealer>().update(|state| state.0 = Some((origin, id)))
    }

    pub fn hide() {
        expect_rw::<Revealer>().update(|state| state.0 = None)
    }
}

pub fn revealer_screen() -> impl IntoView {
    let hidden = move || expect_rw::<Revealer>().with(|state| state.0.is_none());

    view! {
        // Click anywhere else on the screen to dismiss
        <div
            on:click=move |_| { Revealer::hide() }
            class= "fixed z-30 inset-0 cursor-pointer"
            hidden=hidden
        />
    }
}
