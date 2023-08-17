use leptos::*;

use crate::utils::expect_rw;

pub struct Revealer(pub Option<(char, usize)>);

impl Revealer {
    /// Returns true if the current state matches the `origin` and `id` given.
    pub fn state(origin: char, id: usize) -> bool {
        expect_rw::<Revealer>().with(|x| x.0.is_some_and(|(o, i)| o == origin && i == id))
    }

    pub fn open(origin: char, id: usize) {
        expect_rw::<Revealer>().update(|state| state.0 = Some((origin, id)))
    }

    pub fn dismiss() {
        expect_rw::<Revealer>().update(|state| state.0 = None)
    }
}

#[component]
pub fn RevealerScreen() -> impl IntoView {
    let hidden = move || expect_rw::<Revealer>().with(|state| state.0.is_none());

    view! {
        // Click anywhere else on the screen to dismiss
        <div
            on:click=move |_| { Revealer::dismiss() }
            class= "fixed z-30 inset-0 cursor-pointer"
            hidden=hidden
        />
    }
}
