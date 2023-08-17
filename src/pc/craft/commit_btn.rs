use leptos::*;

use super::CraftState;
use crate::svg;
use crate::utils::expect_rw;
use crate::views::toast::Toast;
use crate::{
    pc::{craft::RwProvided, PC},
    views::modal::{ModalCentered, ModalState},
};

pub(super) fn commit_btn() -> impl IntoView {
    let no_chosen_parts =
        move || expect_rw::<CraftState>().with(|x| x.chosen.iter().any(|x| x.is_none()));

    view! {
        <button
            class= "bg-yellow-700 rounded py-1 disabled:bg-zinc-800"
            on:click=move |_| ModalState::open( 2)
            disabled=no_chosen_parts
        >
            <h6> "CRAFT" </h6>
        </button>
    }
}

pub(super) fn craft_item_modal() -> impl IntoView {
    let state = expect_rw::<CraftState>();
    let commit = move |success| {
        commit_recipe(success, state);
        ModalState::dismiss()
    };
    let dc = move || expect_rw::<CraftState>().with(|x| x.dc());

    view! {
        <ModalCentered title=|| "CRAFT ITEM" id=2>
            <div class= "flex-centered relative font-sans text-yellow-600">
                <div class= "fill-black w-24" inner_html=svg::HEXAGON />
                <div class= "absolute">
                    <div class= "text-sm"> "DC" </div>
                    <div class= "text-3xl"> { dc } </div>
                </div>
            </div>
            <button
                class= "border border-green-700 rounded text-green-700 py-2 font-sans"
                on:click=move |_| { commit(true) }
            >
                "SUCCESS"
            </button>
            <button
                class= "border border-red-700 rounded text-red-700 py-2 font-sans"
                on:click=move |_| { commit(false) }
            >
                "FAILURE"
            </button>
        </ModalCentered>
    }
}

/// Removes the a stack of an item with `id` or removes the item
/// if there are no stacks that can be removed.
fn remove_unit(id: usize) {
    PC::update(|pc| {
        if let Some(item) = pc.inventory.get_mut(id) {
            if let Some((curr, _)) = item.stacks.as_mut() {
                if *curr > 1 {
                    *curr -= 1;
                } else {
                    pc.inventory.remove(id);
                }
            } else {
                pc.inventory.remove(id);
            }
        }
    })
}

fn commit_recipe(success: bool, state: RwSignal<CraftState>) {
    let ids: Vec<_> = state.with(|x| x.chosen.iter().flatten().copied().collect());
    let item = state.with(|x| {
        if success {
            x.results().0
        } else {
            x.results().1
        }
        .clone()
    });
    state.update(|x| x.chosen = [None; 2]);
    ids.into_iter().for_each(remove_unit);
    Toast::show(format!("{} added to inventory.", item.name.to_uppercase()));
    PC::update(|pc| pc.inventory.add(item));
}
