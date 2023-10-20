use leptos::*;

use crate::icons;
use crate::utils::rw_utils::RwUtils;
use crate::views::modal::{ModalCenter, ModalState};

pub struct DeleteModal {
    id: usize,
    effect: Box<dyn Fn(usize) + 'static>,
}

impl DeleteModal {
    pub fn show(id: usize) {
        Self::expect().update(|state| state.id = id);
        ModalState::show(0);
    }
    pub fn set_effect<F>(f: F)
    where
        F: Fn(usize) + 'static,
    {
        Self::expect().update(|state| state.effect = Box::new(f));
    }
}

impl Default for DeleteModal {
    fn default() -> Self {
        Self {
            id: 0,
            effect: Box::new(move |_| {}),
        }
    }
}

impl RwUtils for DeleteModal {
    type Item = Self;
}

pub fn delete_confirm_modal() -> impl IntoView {
    let state = DeleteModal::expect();
    let confirm = move |_| {
        state.with(|state| (state.effect)(state.id));
        ModalState::hide();
    };

    view! {
     <ModalCenter id=0>
        <h5 class= "text-center"> "Are you sure?" </h5>
        <div class= "flex gap-2">
            <button
                class= "btn-surface bg-red-800 py-2 basis-1/2 flex-center gap-2"
                on:click=confirm
            >
                <div class= "w-5 -translate-y-[2px]" inner_html=icons::TRASH />
                "DELETE"
                <div class= "w-5 psuedo" />
            </button>
            <button
                class= "btn-surface bg-zinc-700 py-2 basis-1/2"
                on:click=move |_| ModalState::hide()
            >
                "CANCEL"
            </button>
        </div>
     </ModalCenter>
    }
}
