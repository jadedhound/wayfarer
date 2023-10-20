use leptos::*;

use crate::utils::expect_rw;
use crate::utils::rw_utils::RwUtils;

#[derive(Default)]
pub struct Revealer(pub Option<(char, usize)>);

impl Revealer {
    /// Returns true if the current state matches the `origin` and `id` given.
    pub fn shown() -> bool {
        Revealer::expect().with(|x| x.0.is_some())
    }

    /// Returns true if the current state matches the `origin` and `id` given.
    pub fn is_shown(origin: char, id: usize) -> bool {
        Revealer::expect().with(|x| x.0.is_some_and(|(o, i)| o == origin && i == id))
    }

    /// Returns true if the current state DOES NOT match the `origin` and `id` given.
    pub fn hidden(origin: char, id: usize) -> bool {
        !Self::is_shown(origin, id)
    }

    pub fn show(origin: char, id: usize) {
        Revealer::expect().update(|state| state.0 = Some((origin, id)))
    }

    pub fn hide() {
        Revealer::expect().update(|state| state.0 = None)
    }
}

impl RwUtils for Revealer {
    type Item = Self;
}

pub fn revealer_screen() -> impl IntoView {
    let hidden = move || expect_rw::<Revealer>().with(|state| state.0.is_none());

    view! {
        <button
            on:click=move |_| { Revealer::hide() }
            class= "fixed z-30 h-full w-full top-0"
            hidden=hidden
        />
    }
}
