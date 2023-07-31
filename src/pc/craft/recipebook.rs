use std::cmp::min;

use const_format::concatcp;
use leptos::*;

use crate::items::{recipes as rcp, Item};
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::utils::rw_context;
use crate::views::modal::{ModalCentered, ModalState};
use crate::{css, svg};

#[component]
pub(super) fn Recipebook(cx: Scope) -> impl IntoView {
    let pc = rw_context::<PC>(cx);
    let book = move || {
        pc.with(|pc| {
            pc.recipes
                .iter()
                .enumerate()
                .map(|(i, maybe_r)| match maybe_r {
                    Some(recipe) => view! { cx, <Recipe i recipe /> },
                    None => view! { cx, <EmptyRecipe /> },
                })
                .collect_view(cx)
        })
    };

    view! {
        cx,
        <ModalCentered title=|| "RECIPEBOOK" id=0>
            <div class= "px-2 grid grid-cols-2 gap-2 h-[60vh] auto-rows-fr">
                { book }
            </div>
        </ModalCentered>
    }
}

enum HasIngredients {
    All,
    Some,
    None,
}

fn num_in_inv(cx: Scope, item: &Item) -> u8 {
    rw_context::<PC>(cx).with_untracked(|pc| {
        pc.inventory
            .iter()
            .filter(|x| *x == item)
            .fold(0_u8, |acc, e| {
                let stack = e.spec.as_stackable().map(|(curr, _)| *curr);
                acc + stack.unwrap_or(1)
            })
    })
}

fn has_ingredients(cx: Scope, recipe: &rcp::Recipe) -> HasIngredients {
    let mut max = 0;
    let mut hits = 0;
    for (item, num) in recipe.ingredients.iter().flatten() {
        max += num;
        hits += min(num_in_inv(cx, item), *num);
    }
    if hits == max {
        HasIngredients::All
    } else if hits > 0 {
        HasIngredients::Some
    } else {
        HasIngredients::None
    }
}

#[component]
fn Recipe<'a>(cx: Scope, i: usize, recipe: &'a rcp::Recipe) -> impl IntoView {
    let name = recipe.success.name.clone();
    let (colour, icon) = match has_ingredients(cx, recipe) {
        HasIngredients::All => ("bg-green-700", svg::TICK),
        HasIngredients::Some => ("bg-yellow-700", svg::EXCLAMATION),
        HasIngredients::None => ("bg-red-700", svg::CROSS),
    };

    view! {
        cx,
        <button
            class= concatcp!(css::BTN, " relative")
            on:click=move |_| {
                rw_context::<PCSession>(cx).update(|sesh| sesh.rcp_index = i);
                ModalState::dismiss(cx)
            }
        >
            <div class=format!("top-0 right-0 -translate-y-1 translate-x-1 absolute rounded-full {colour} w-6 h-6 flex-centered")>
                <div class= "svg w-4" inner_html=icon />
            </div>
            <div class= "font-sans"> { name.to_uppercase() } </div>
        </button>
    }
}

#[component]
fn EmptyRecipe(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class= "psuedo border-2 border-zinc-700 rounded" />
    }
}

#[component]
fn NewRecipe<'a>(cx: Scope, maybe_r: Option<&'a rcp::Recipe>) -> impl IntoView {
    view! {
        cx,
        { match maybe_r {
            Some(_x) => view!{ cx,
                <div />
            },
            None => view!{ cx,
                <div class= "psuedo border-2 border-yellow-700 rounded w-1/2 h-full flex-centered">
                </div>
            }
        }}
    }
}
