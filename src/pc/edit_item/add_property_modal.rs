use leptos::*;

use super::State;
use crate::items::ItemProp as Prop;
use crate::utils::counter::Counter;
use crate::utils::rw_utils::RwUtils;
use crate::views::modal::{ModalCenter, ModalLocation, ModalState};

/// Props that can be added by the user.
fn addable_props() -> [Prop; 9] {
    [
        Prop::Bulky,
        Prop::Concentration,
        Prop::Count(Counter::new(2)),
        Prop::Damage(1),
        Prop::Effect(String::new()),
        Prop::Range(30),
        Prop::Resist,
        Prop::Usable(String::new()),
        Prop::Passive,
    ]
}

pub fn open_add_property() -> impl IntoView {
    let open_modal = move |_| ModalState::show(ModalLocation::AddItemProp);
    let disabled = State::slice(|state| {
        let curr_indexes = curr_indexes(state);
        // Disabled if there is no first instance found.
        !addable_props()
            .into_iter()
            // Find the first instance of a `prop.index` NOT being in `curr_indexes`.
            .any(|prop| !curr_indexes.contains(&prop.index()))
    });

    view! {
        <button
            class= "btn bg-surface"
            on:click=open_modal
            disabled=disabled
        >
            "ADD PROPERTY"
        </button>
    }
}

pub fn add_property_modal() -> impl IntoView {
    let state = State::expect();
    let props = move || {
        let curr_indexes = state.with(curr_indexes);
        addable_props()
            .into_iter()
            // Make sure we don't allow props that the item already has to be addable.
            .filter(|prop| !curr_indexes.contains(&prop.index()))
            .map(add_prop)
            .collect_view()
    };
    view! {
        <ModalCenter location=ModalLocation::AddItemProp>
            <h4 class= "text-center"> "Add Property" </h4>
            { props }
        </ModalCenter>
    }
}

fn curr_indexes(state: &State) -> Vec<usize> {
    state.item.props.iter().map(|prop| prop.index()).collect()
}

fn add_prop(prop: Prop) -> impl IntoView {
    let state = State::expect();
    let prop_clone = prop.clone();
    let add_prop = move |_| {
        let prop_clone = prop_clone.clone();
        state.update(|state| state.item.props.push(prop_clone));
        ModalState::hide();
    };

    view! {
        <button
            class= "btn-surface bg-zinc-700"
            on:click=add_prop
        >
            { prop.to_string() }
        </button>
    }
}
