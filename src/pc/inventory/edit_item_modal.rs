use leptos::*;

use crate::icons;
use crate::items::Item;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::views::funds::{maybe_funds, wealth_input};
use crate::views::modal::{ModalCustom, ModalState};

#[derive(Default, Clone)]
pub struct State {
    pub id: usize,
    pub item: Item,
}

impl RwUtils for State {
    type Item = State;
}

pub fn edit_item_modal() -> impl IntoView {
    let state = State::expect();
    let item = move || state.with(|state| state.item.clone()).into_view();
    let price_preview = move || maybe_funds(state.with(|state| state.item.price()));
    view! {
        <ModalCustom id=10>
            <div class= "flex flex-col gap-2 bg-zinc-900 h-full p-2 relative animate-popin overflow-y-auto">
                <button
                    class= "absolute top-2 right-2 btn bg-red-800 p-2"
                    on:click=move |_| ModalState::hide()
                >
                    <div class= "w-4" inner_html=icons::CROSS />
                </button>
                <h4 class= "text-center"> "Preview" </h4>
                <div class= "border-y-2 border-emerald-500 py-2 flex flex-col">
                    { item }
                    { price_preview }
                </div>
                <h4 class= "text-center"> "Properties" </h4>
                <div class= "grid grid-cols-5 gap-2">
                    { name }
                    { price }
                    { desc }
                </div>
                { apply_button }
            </div>
        </ModalCustom>
    }
}

fn name() -> impl IntoView {
    let (name, name_set) = create_slice(
        State::expect(),
        |state| state.item.name.clone(),
        |state, value| state.item.name = value,
    );
    view! {
        <div class= "font-tight self-center"> "NAME" </div>
        <input
            class= "col-span-4 input"
            on:input=move |ev| name_set.set(event_target_value(&ev))
            prop:value=name
        />
    }
}

fn price() -> impl IntoView {
    let state = State::expect();
    let price = RwSignal::new(0);
    let curr = State::slice(|state| state.item.base_price);
    // When item updates, ensure the new price is used.
    create_effect(move |_| {
        let curr = curr.get();
        if curr != price.get_untracked() {
            price.set(curr)
        }
    });
    create_effect(move |_| {
        let price = price.get();
        if price != curr.get_untracked() {
            state.update(|state| state.item.base_price = price)
        }
    });
    view! {
        <div class= "font-tight self-center"> "PRICE" </div>
        <div class= "col-span-4"> { wealth_input(price) } </div>
    }
}

fn desc() -> impl IntoView {
    let (desc, desc_set) = State::rw_slice(
        |state| state.item.desc.clone(),
        |state, value| state.item.desc = value,
    );
    view! {
        <textarea
            class= "input col-span-5 h-24"
            on:input=move |ev| desc_set.set(event_target_value(&ev))
            prop:value=desc
        />
    }
}

fn apply_button() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let save_item = move |_| {
        let State { id, item } = state.get();
        pc.update(|pc| *pc.inventory.get_mut(id).unwrap() = item);
        ModalState::hide();
    };
    view! {
        <button
            class= "btn-surface bg-green-800 py-2"
            on:click=save_item
        >
            "APPLY"
        </button>
    }
}
