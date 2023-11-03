use leptos::*;

use crate::utils::rw_utils::RwUtils;

#[derive(Clone, Copy, Default)]
pub struct ModalState(Option<ModalLocation>);

#[derive(Clone, Copy, PartialEq)]
pub enum ModalLocation {
    CreatePC,
    ClassOptionalBuff,
    AddItemProp,
    ShopItemDetails,
    DeleteConfirm,
    RecentlyRemoved,
}

impl ModalState {
    pub fn show(location: ModalLocation) {
        let stop_scroll = document()
            .body()
            .and_then(|body| body.class_list().add_1("no-scroll").ok());
        if stop_scroll.is_none() {
            logging::error!("Unable to restrict scrolling");
        }
        ModalState::expect().update(|state| state.0 = Some(location))
    }
    pub fn hide() {
        let start_scroll = document()
            .body()
            .and_then(|body| body.class_list().remove_1("no-scroll").ok());
        if start_scroll.is_none() {
            logging::error!("Unable to enable scrolling");
        }
        ModalState::expect().update(|state| state.0 = None)
    }
}

impl RwUtils for ModalState {}

pub fn modal_grey_screen() -> impl IntoView {
    let hidden = move || ModalState::expect().get().0.is_none();

    view! {
        <div
            class= "fixed top-0 left-0 h-full w-full z-20 bg-zinc-800 bg-opacity-75"
            hidden=hidden
        />
    }
}

/// A centered popup modal.
/// `id` range 0-9 is reserved for static modals.
#[component]
pub fn ModalCenter(children: Children, location: ModalLocation) -> impl IntoView {
    let hidden = move || !ModalState::expect().get().0.is_some_and(|x| x == location);
    let dismiss = move |_| ModalState::hide();

    view! {
        <div class= "fixed top-0 left-0 h-full w-full flex flex-col z-[21]" hidden=hidden>
            <button class= "h-px grow" on:click=dismiss />
            <div class= "animate-popin p-2">
                <div
                    class= "bg-surface shadow-md shadow-black rounded
                    inset-0 p-4 overflow-y-auto max-h-[70vh] flex flex-col gap-2"
                >
                    { children() }
                </div>
            </div>
            <button class= "h-px grow" on:click=dismiss />
        </div>
    }
}

#[component]
pub fn ModalCustom(children: Children, location: ModalLocation) -> impl IntoView {
    let hidden = move || !ModalState::expect().get().0.is_some_and(|x| x == location);

    view! {
        <div class= "relative z-[21]" hidden=hidden>
            <div class= "fixed top-0 left-0 h-full w-full">
                { children() }
            </div>
        </div>
    }
}
