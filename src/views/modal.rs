use std::fmt::Display;

use leptos::*;

use crate::utils::rw_context;

pub struct ModalState(Option<u8>);

impl ModalState {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn open(cx: Scope, id: u8) {
        rw_context::<ModalState>(cx).update(|state| state.0 = Some(id))
    }

    pub fn dismiss(cx: Scope) {
        rw_context::<ModalState>(cx).update(|state| state.0 = None)
    }
}

#[component]
fn GreyScreen(cx: Scope, children: Children, id: u8) -> impl IntoView {
    let hidden = move || rw_context::<ModalState>(cx).with(|state| state.0 != Some(id));
    view! {
        cx,
        <div class= "fixed inset-0 z-50" hidden=hidden>
            <div class= "h-full bg-zinc-800 bg-opacity-75"/>
                { children(cx) }
        </div>
    }
}

#[component]
pub fn ModalCentered<F, S>(cx: Scope, children: Children, title: F, id: u8) -> impl IntoView
where
    F: Fn() -> S + 'static,
    S: Display,
{
    view! {
        cx,
        <GreyScreen id=id>
            <div class= "absolute inset-0 w-full flex flex-col">
                <div class= "h-12 grow" on:click=move |_| ModalState::dismiss(cx) />
                <div class= "w-full px-2 animate-popin overflow-auto">
                    <div class= "bg-zinc-800 rounded shadow-sm shadow-zinc-900 h-full w-full text-center flex flex-col p-4 gap-2 overflow-auto">
                        <h4 class= "border-b-2 border-purple-700 mb-2"> { move || title().to_string() } </h4>
                        { children(cx) }
                    </div>
                </div>
                <div class= "h-12 grow" on:click=move |_| ModalState::dismiss(cx) />
            </div>
        </GreyScreen>
    }
}
