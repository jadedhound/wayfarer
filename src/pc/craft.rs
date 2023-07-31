pub mod recipe;
pub mod recipebook;
use leptos::*;

use crate::pc::craft::recipe::CraftRecipe;
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::utils::rw_context;

#[component]
pub fn Craft(cx: Scope) -> impl IntoView {
    let pc = rw_context::<PC>(cx);
    let sesh = rw_context::<PCSession>(cx);
    let recipe = move || {
        let i = sesh.with(|s| s.rcp_index);
        pc.with(|pc| pc.recipes.get(i).cloned()).flatten()
    };

    view! { cx,
        { move || match recipe() {
            None => view!{ cx, <NoRecipes /> }.into_view(cx),
            Some(recipe) => view!{ cx, <CraftRecipe recipe /> }.into_view(cx)
        }}
    }
}

#[component]
fn NoRecipes(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class= "grow flex-centered">
            <h4> "NO RECIPES FOUND" </h4>
        </div>
    }
}
