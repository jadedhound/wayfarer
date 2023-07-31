use const_format::concatcp;
use leptos::*;

use crate::items::item_specs::ItemSpec;
use crate::items::recipes::Recipe;
use crate::items::Item;
use crate::pc::craft::recipebook::Recipebook;
use crate::pc::equip_slot::EquipSlot;
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::svg;
use crate::utils::rw_context;
use crate::views::modal::ModalState;
use crate::views::toast::{Toast, ToastNotif};

struct HasIngredients(bool);

#[component]
pub(super) fn CraftRecipe(cx: Scope, recipe: Recipe) -> impl IntoView {
    let has_ingredients = create_rw_signal(cx, HasIngredients(true));
    provide_context(cx, has_ingredients);

    let ingredients = 
        // TODO: Try remove this clone
        recipe.ingredients.map(|x| {
            if let Some((item, num)) = x {
                view! { cx,
                    <Ingredient item=item num />
                }
                .into_view(cx)
            } else {
                ().into_view(cx)
            }
        })
    ;

    view! {
        cx,
        <div class= "flex flex-col gap-2 px-2">
            <h5 class= "border-b border-yellow-500 text-center"> "CRAFTING" </h5>
            <ChooseRecipeBtn item=&recipe.success />
            <h5 class= "grid grid-cols-2 gap-x-4 my-4">
                <div class= "text-right"> "DC" </div>
                <DC item=&recipe.success />
                <div class= "text-right"> "Time" </div>
                <div> "N/A" </div>
            </h5>
            <h5 class= "border-b border-yellow-500 text-center"> "RESOURCES" </h5>
            <div class= "grid grid-cols-2 gap-x-2">
                { ingredients }
            </div>
            <CommitBtn />
            <div class= "pseudo h-6" />
        </div>
        <ToastNotif />
        <Recipebook />
    }
}

#[component]
fn Ingredient(cx: Scope, item: Item, num: u8) -> impl IntoView {
    let pc = rw_context::<PC>(cx);
    let inv_num = pc.with_untracked(|pc| {
        pc.inventory
            .iter()
            .filter(|x| **x == item)
            .fold(0_u8, |acc, e| {
                let stack = e.spec.as_stackable().map(|(curr, _)| *curr);
                acc + stack.unwrap_or(1)
            })
    });

    view! {
        cx,
        <div>
            { if inv_num >= num {
                view!{ cx,
                    <div class= "rounded border-green-700 border-2 relative">
                        { item.into_view(cx) }
                        <div class= "absolute w-6 h-6 -translate-y-1 translate-x-1 top-0 right-0 rounded-full bg-green-700 flex-centered">
                            <div class= "svg w-3" inner_html=svg::TICK />
                        </div>
                    </div>
                }
            } else {
                rw_context::<HasIngredients>(cx).update(|x| x.0 = false);
                view!{ cx,
                    <div class= "rounded border-2 border-red-700 relative">
                        { item.into_view(cx) }
                        <div class= "absolute w-6 h-6 -translate-y-1 translate-x-1 top-0 right-0 rounded-full bg-red-700 flex-centered">
                            <div class= "svg w-3" inner_html=svg::CROSS />
                        </div>
                    </div>
                }
            }}
            <div class= "text-center font-sans"> { format!("{inv_num} / {num}") } </div>
        </div>
    }
}

#[component]
fn ChooseRecipeBtn<'a>(cx: Scope, item: &'a Item) -> impl IntoView {
    view! {
        cx,
        <button 
            class= "bg-zinc-800 rounded"
            on:click=move |_| ModalState::open(cx, 0)
        >
            { item.into_view(cx) }
        </button>
    }
}

#[component]
fn DC<'a>(cx: Scope, item: &'a Item) -> impl IntoView {
    let pc = rw_context::<PC>(cx);
    let tool_dc = move || {
        pc.with(|pc| {
            pc.equipment[EquipSlot::Tools.index()]
                .as_ref()
                .map(|x| (x.quality as u8 + 1) * 5)
                .unwrap_or(0)
        })
    };
    let craft_dc = 10 + (item.quality as u8 * 5);
    view! {
        cx,
        <div> { move || craft_dc - tool_dc() } </div>
    }
}

enum Outcome {
    CritSuccess,
    Success,
    Failure,
}

/// Removes the number of stacks indicated by `num`. Removes
/// multiple items if the item in question doesn't stack.
fn remove_stacks(cx: Scope, item: &Item, num: u8) {
    let pc = rw_context::<PC>(cx);
    let mut num = num;
    pc.update(|pc| {
        let inv_clone = pc.inventory.clone();
        let filtered = inv_clone
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, x)| *x == item);
        for (i, item) in filtered {
            if let Some((curr, max)) = item.spec.as_stackable() {
                // Remove entire stack.
                if num >= *curr {
                    pc.inventory.remove(i);
                    num -= curr;
                // Remove only the required amount of stacks.
                } else {
                    let item = &mut pc.inventory[i];
                    item.spec = ItemSpec::Stackable(curr - num, *max);
                    num = 0;
                }
            } else {
                pc.inventory.remove(i);
                num -= 1;
            }
            // Early stop if ammount has been deducted.
            if num < 1 {
                break;
            }
        }
    })
}

fn commit_recipe(cx: Scope, outcome: Outcome) {
    let pc = rw_context::<PC>(cx);
    let Recipe {
        ingredients,
        success,
        failure,
    } = rw_context::<PCSession>(cx)
        .with(|sesh| pc.with(|pc| pc.recipes[sesh.rcp_index].clone()))
        .unwrap();
    for (item, num) in ingredients.into_iter().flatten() {
        remove_stacks(cx, &item, num)
    }
    let item = match outcome {
        Outcome::CritSuccess => success,
        Outcome::Success => success,
        Outcome::Failure => failure,
    };
    Toast::show(cx, format!("Added {} to inventory!", item.name));
    pc.update(|pc| pc.add_inv_item(&item));
}

#[component]
fn CommitBtn(cx: Scope) -> impl IntoView {
    const NORMAL: &str = " bg-zinc-700";
    const DISABLED: &str = " disabled:border-2 disabled:border-zinc-700 disabled:bg-inherit";
    const COMMON: &str = "h-12 font-sans rounded w-full";

    let popup_hidden = create_rw_signal(cx, true);

    view! { cx,
        <div class= "relative mt-6">
            <button
                class=concatcp!(COMMON, NORMAL, DISABLED)
                on:click=move |_| {
                    popup_hidden.set(false);
                }
                disabled=move || {
                    rw_context::<HasIngredients>(cx).with(|x| !x.0)
                }
                hidden=move || !popup_hidden.get()
            >
                "COMMIT"
            </button>
            <div
                class= "pseudo cursor-pointer fixed inset-0"
                on:click=move |_| {
                    popup_hidden.set(true);
                }
                hidden=move || popup_hidden.get()
            />
            <div 
                class= "absolute inset-0 space-y-4 z-10"
                hidden=move || popup_hidden.get()
            >
                <button
                    class=concatcp!(COMMON, " bg-red-700")
                    on:click=move |_| { commit_recipe(cx, Outcome::Failure) }
                >
                    "FAILURE"
                </button>
                <button
                    class=concatcp!(COMMON, NORMAL)
                    on:click=move |_| { commit_recipe(cx, Outcome::Success) }
                >
                    "SUCCESS"
                </button>
                <button
                    class=concatcp!(COMMON, NORMAL)
                    on:click=move |_| { commit_recipe(cx, Outcome::CritSuccess) }
                >
                    "CRITICAL SUCCESS"
                </button>
                <div class= "pseudo h-6" />
            </div>
        </div>
    }
}
