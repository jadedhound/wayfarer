use leptos::*;

use crate::items::reagents::Substance;
use crate::items::recipes;
use crate::pc::craft::CraftState;
use crate::pc::pc_stat::PCStat;
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::svg;
use crate::utils::{expect_rw, some_if, RwProvided};
use crate::views::modal::{ModalCentered, ModalState};

pub(super) fn choose_recipe_btn() -> impl IntoView {
    let state = expect_rw::<CraftState>();
    let name = move || state.with(|state| state.recipe.name.clone());

    view! {
        <button
            class= "px-2 py-4 text-left flex items-center gap-2 font-sans"
            on:click=move |_| ModalState::open( 0)
        >
            <div class= "svg w-6" inner_html=svg::BOOK />
            <div class= "ml-2 w-12 grow uppercase"> { name } </div>
            <div class= "svg w-4" inner_html=svg::QUILL />
        </button>
    }
}

pub(super) fn recipebook() -> impl IntoView {
    let book = move || {
        PC::with(|pc| {
            pc.recipes
                .iter()
                .enumerate()
                .map(|(i, rcp)| recipe(i, rcp))
                .collect_view()
        })
    };
    let empty = move || {
        PCSession::with_pc(|sesh, pc| {
            let missing = sesh.stats.get(PCStat::Recipes) - pc.recipes.len() as i32;
            some_if(missing > 0).map(|_| (0..missing).map(|_| empty_recipe()).collect_view())
        })
    };

    view! {
        <ModalCentered title=|| "RECIPEBOOK" id=0>
            <div class= "flex flex-col gap-1 shaded-table-light font-sans">
                { book }
                { empty }
            </div>
        </ModalCentered>
    }
}

fn empty_recipe() -> impl IntoView {
    view! {
        <div class= "psuedo h-14" />
    }
}

fn recipe(i: usize, recipe: &recipes::Recipe) -> impl IntoView {
    let name = recipe.name.clone();
    let warn = some_if(not_enough_substances(&recipe.substances)).map(|_| {
        view! {
            <div class= "w-4" inner_html=svg::CAUTION />
        }
    });

    view! {
        <div class= "flex items-center px-2 gap-2">
            { warn }
            <button
                class= "text-left py-4 w-12 grow uppercase"
                on:click=move |_| {
                    PCSession::update( |sesh| sesh.recipe_index = i);
                    ModalState::dismiss()
                }
            >
                { name }
            </button>
        </div>
    }
}

/// Checks whether the recipe can even be created.
fn not_enough_substances(substances: &[Substance]) -> bool {
    let num_in_inv = |sub| {
        PC::with(|pc| {
            pc.inventory
                .values()
                .filter_map(|x| x.spec.as_reagent()?.get(sub).map(|_| x))
                .fold(0, |acc, e| {
                    let stack = e.stacks.map(|(curr, _)| curr);
                    acc + stack.unwrap_or(1)
                })
        })
    };

    if substances[0] == substances[1] {
        num_in_inv(substances[0]) < 2
    } else {
        num_in_inv(substances[0]) < 1 && num_in_inv(substances[1]) < 1
    }
}
