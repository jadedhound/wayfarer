use leptos::*;
use leptos_router::{use_location, use_navigate, use_params_map};

use self::add_property_modal::{add_property_modal, open_add_property};
use self::edit_props::edit_props;
use crate::icons;
use crate::items::Item;
use crate::pc::PC;
use crate::utils::counter::Counter;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;
use crate::views::revealer::Revealer;
use crate::views::wealth::{maybe_wealth, wealth_input};

mod add_property_modal;
mod edit_props;

#[derive(Clone)]
struct State {
    id: usize,
    item: Item,
}

pub fn edit_item() -> impl IntoView {
    // The Revealer shown for backpack::more_button needs to be hidden again.
    Revealer::hide();
    State::provide();

    view! {
        <h4 class= "text-center"> "Preview" </h4>
        { item_preview }
        <h4 class= "text-center mt-2"> "Base" </h4>
        <div class= "grid grid-cols-6 gap-2">
            { name }
            { price }
        </div>
        <h4 class= "text-center col-span-6 mt-2"> "Properties" </h4>
        { edit_props }
        { open_add_property }
        { apply_button }
        { add_property_modal }
    }
}

fn item_preview() -> impl IntoView {
    let state = State::expect();
    let item = move || state.with(|state| state.item.clone()).into_view();
    let price_preview = move || maybe_wealth(state.with(|state| state.item.base_price));
    let count = move |count: Counter| {
        view! {
            <div class= "flex items-center justify-center gap-2">
                <div> { count.max } </div>
                <div class= "w-4" inner_html=icons::STACK />
            </div>
        }
    };
    let maybe_count =
        move || state.with(|state| state.item.find_counter().map(count).collect_view());

    view! {
        <div class= "border-y-2 border-emerald-500 py-2 flex flex-col">
            { item }
            <div class= "flex items-center">
                { price_preview }
                <div class= "psuedo w-12" />
                { maybe_count }
            </div>
        </div>
    }
}

fn name() -> impl IntoView {
    let (name, name_set) = create_slice(
        State::expect(),
        |state| state.item.name.clone(),
        |state, value| state.item.name = value,
    );
    view! {
        <div class= "font-tight self-center col-span-2"> "NAME" </div>
        <input
            class= "col-span-4 input"
            on:input=move |ev| name_set.set(event_target_value(&ev))
            prop:value=name
        />
    }
}

fn price() -> impl IntoView {
    let state = State::expect();
    let price = RwSignal::new(state.with_untracked(|state| state.item.base_price));
    create_effect(move |_| {
        let price = price.get();
        state.update(|state| state.item.base_price = price)
    });

    view! {
        <div class= "font-tight self-center col-span-2"> "PRICE" </div>
        <div class= "col-span-4"> { wealth_input(price) } </div>
    }
}

fn apply_button() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let nav_to_inventory = move || {
        let path = use_location().pathname.get_untracked();
        let url: Vec<&str> = path.split('/').take(3).collect();
        let url = format!("{}/main", url.join("/"));
        (use_navigate())(&url, Default::default());
    };
    let save_item = move |_| {
        let State { id, item } = state.get();
        // Removing and adding to trigger relevant update functions.
        pc.update_discard(|pc| pc.inventory.remove(id));
        pc.update_discard(|pc| pc.inventory.add(item));
        nav_to_inventory();
    };
    view! {
        <button
            class= "btn-surface bg-green-800"
            on:click=save_item
        >
            "APPLY"
        </button>
    }
}

// -----------------------------------
// TRAIT IMPLS
// -----------------------------------

impl Default for State {
    fn default() -> Self {
        let pc = PC::expect();
        let id = use_params_map()
            .get()
            .get("id")
            .and_then(|id| id.parse::<usize>().ok())
            .unwrap_or_default();
        let item = pc.with_untracked(|pc| pc.inventory.get(id).cloned().unwrap_or_default());
        Self { id, item }
    }
}

impl RwUtils for State {}
