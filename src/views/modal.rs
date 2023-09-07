use std::fmt::Display;

use leptos::*;

use crate::utils::expect_rw;

pub struct ModalState(Option<u8>);

impl ModalState {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn open(id: u8) {
        expect_rw::<ModalState>().update(|state| state.0 = Some(id))
    }

    pub fn dismiss() {
        expect_rw::<ModalState>().update(|state| state.0 = None)
    }

    pub fn get() -> Option<u8> {
        expect_rw::<ModalState>().with(|x| x.0)
    }
}

pub fn modal_grey_screen() -> impl IntoView {
    let hidden = move || expect_rw::<ModalState>().with(|state| state.0.is_none());

    view! {
        <div class= "fixed inset-0 z-30" hidden=hidden>
            <div class= "h-full bg-zinc-800 bg-opacity-75"/>
        </div>
    }
}

#[component]
pub fn CenterModal<F, S>(children: Children, title: F, id: u8) -> impl IntoView
where
    F: Fn() -> S + 'static,
    S: Display,
{
    let hidden = move || !expect_rw::<ModalState>().with(|state| state.0 == Some(id));

    view! {
        <div class= "relative z-40" hidden=hidden>
            <div class= "fixed top-0 h-full w-full flex flex-col">
                <div class= "h-[15vh] grow" on:click=move |_| ModalState::dismiss() />
                <div class= "w-full p-2 animate-popin overflow-auto">
                    <div class= "bg-surface shadow-md shadow-black rounded h-full w-full text-center flex flex-col p-4 gap-4 overflow-auto">
                        <h5 class= "uppercase"> { move || title().to_string() } </h5>
                        { children() }
                    </div>
                </div>
                <div class= "pseudo h-[15vh] grow" on:click=move |_| ModalState::dismiss() />
            </div>
        </div>
    }
}
