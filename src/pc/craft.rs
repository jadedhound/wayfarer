mod commit_btn;
pub mod recipe;
pub mod recipebook;
mod substance_btn;
use leptos::*;

use crate::items::recipes::Recipe;
use crate::items::Item;
use crate::pc::craft::recipe::craft_recipe;
use crate::pc::session::PCSession;
use crate::utils::RwProvided;

use super::PC;

pub(super) struct CraftState {
    pub recipe: Recipe,
    pub chosen: [Option<usize>; 2],
    pub change_first: bool,
}

impl CraftState {
    fn new(recipe: Recipe) -> Self {
        Self {
            recipe,
            chosen: [None; 2],
            change_first: true,
        }
    }

    fn dc_points(&self) -> u8 {
        let get_points = |id, sub| {
            PC::with(|pc| {
                let item = pc.inventory.get(id);
                item.and_then(|x| x.spec.as_reagent()?.get(sub))
            })
        };
        let subs = self.recipe.substances;
        let x = self.chosen[0]
            .and_then(|id| get_points(id, subs[0]))
            .unwrap_or(0);
        let y = self.chosen[1]
            .and_then(|id| get_points(id, subs[1]))
            .unwrap_or(0);
        x + y
    }

    fn dc(&self) -> u8 {
        let points = std::cmp::min(self.dc_points(), 59);
        // Range from 15 - 5.
        (30 - (points % 20)) / 2
    }

    /// Calculates the `success` and `failure` items.
    fn results(&self) -> (&Item, &Item) {
        let points = std::cmp::min(self.dc_points(), 59);
        // The success item is index min 1 and max 3.
        let success_i = (points as usize / 20) + 1;
        let success = self.recipe.products.get(success_i).unwrap();
        let failure = self.recipe.products.get(success_i - 1).unwrap();
        (success, failure)
    }
}

pub fn craft() -> impl IntoView {
    let recipe = move || {
        PCSession::with_pc(|sesh, pc| {
            let i = sesh.recipe_index;
            pc.recipes.get(i).or_else(|| pc.recipes.first()).cloned()
        })
    };

    match recipe() {
        None => no_recipes().into_view(),
        Some(recipe) => {
            provide_context(create_rw_signal(CraftState::new(recipe.clone())));
            craft_recipe().into_view()
        }
    }
}

fn no_recipes() -> impl IntoView {
    view! { cx,
        <div class= "grow flex-centered">
            <h4> "NO RECIPES FOUND" </h4>
        </div>
    }
}
