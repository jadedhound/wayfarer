use gloo::events::EventListener;
use leptos::*;

use crate::utils::rw_utils::RwUtils;
use crate::utils::{expect_rw, RwSignalEnhance};

#[derive(PartialEq, Clone, Copy)]
pub enum RevLocation {
    BackpackMore,
    CountButton,
    SellConfirm,
    RallyConfirm,
    ShopBuy,
    RestConfirm,
    SettingDatabase,
}

#[derive(Default)]
pub struct Revealer(Option<(RevLocation, usize, EventListener)>);

impl Revealer {
    /// Returns true if the current state matches the `location` and `id` given.
    pub fn is_shown(loc: RevLocation, id: usize) -> bool {
        Revealer::expect().with(|state| {
            state
                .0
                .as_ref()
                .is_some_and(|(curr_loc, curr_id, _)| curr_loc == &loc && curr_id == &id)
        })
    }
    /// Returns true if the current state DOES NOT match the `location` and `id` given.
    pub fn is_hidden(location: RevLocation, id: usize) -> bool {
        !Self::is_shown(location, id)
    }

    pub fn show(location: RevLocation, id: usize) {
        let target = web_sys::window().unwrap();
        let listener = EventListener::new(&target, "scroll", move |_| Revealer::hide());
        Revealer::expect().update(|state| state.0 = Some((location, id, listener)))
    }

    pub fn hide() {
        Revealer::expect().reset()
    }
}

impl RwUtils for Revealer {}

pub fn revealer_screen() -> impl IntoView {
    let hidden = move || expect_rw::<Revealer>().with(|state| state.0.is_none());

    view! {
        <button
            on:click=move |_| { Revealer::hide() }
            class= "fixed z-30 h-full w-full top-0 left-0"
            hidden=hidden
        />
    }
}
