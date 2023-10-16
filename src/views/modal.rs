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
    let hidden = move || ModalState::get().is_none();

    view! {
        <div class= "fixed inset-0 z-30" hidden=hidden>
            <div class= "h-full bg-zinc-800 bg-opacity-75"/>
        </div>
    }
}

#[component]
pub fn ModalCenter(children: Children, id: u8) -> impl IntoView {
    let hidden = move || !ModalState::get().is_some_and(|x| x == id);
    let dismiss = move |_| ModalState::dismiss();

    view! {
        <div class= "relative z-40" hidden=hidden>
            <div class= "fixed top-0 left-0 h-full w-full flex flex-col">
                <div class= "h-[15vh] grow" on:click=dismiss />
                <div class= "animate-popin p-2">
                    <div class= "bg-surface shadow-md shadow-black rounded h-full w-full p-4 overflow-y-auto">
                        { children() }
                    </div>
                </div>
                <div class= "h-[15vh] grow" on:click=dismiss />
            </div>
        </div>
    }
}

#[component]
pub fn ModalCenterCustom(children: Children, id: u8) -> impl IntoView {
    let hidden = move || !ModalState::get().is_some_and(|x| x == id);
    let dismiss = move |_| ModalState::dismiss();

    view! {
        <div class= "relative z-40" hidden=hidden>
            <div class= "fixed top-0 left-0 h-full w-full flex flex-col">
                <div class= "h-[15vh] grow" on:click=dismiss />
                <div class= "animate-popin p-2">
                    { children() }
                </div>
                <div class= "h-[15vh] grow" on:click=dismiss />
            </div>
        </div>
    }
}
