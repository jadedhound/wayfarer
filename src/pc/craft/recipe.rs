use leptos::*;

use crate::pc::craft::recipebook::choose_recipe_btn;
use crate::utils::expect_rw;

use super::commit_btn::{commit_btn, craft_item_modal};
use super::recipebook::recipebook;
use super::substance_btn::{modal_substance_filter, substance_view};
use super::CraftState;

pub(super) fn craft_recipe() -> impl IntoView {
    let state = expect_rw::<CraftState>();

    view! {
        <div class= "flex flex-col gap-4 px-2">
            <h5 class= "text-center"> "RESOURCES" </h5>
            <div class= "flex flex-col shaded-table">
                { choose_recipe_btn() }
                { substance_view( true) }
                { substance_view( false) }
            </div>
            <h5 class= "text-center"> "RESULT" </h5>
            <div class= "flex flex-col gap-2">
                <div class= "flex">
                    <div class= "w-28">
                        "Quality"
                    </div>
                    { dc_slider() }
                </div>
                <div class= "flex">
                    <div class= "w-28">
                        "Difficulty"
                    </div>
                    <div class= "w-full text-center font-sans">
                        "DC " { move || state.with(|state| state.dc()) }
                    </div>
                </div>
            </div>
            <div class= "border-y border-yellow-600 py-2">
                { move || state.with(|state| state.results().0.into_view()) }
            </div>
            { commit_btn() }
            <div class= "psuedo h-6" />
        </div>
        { recipebook() }
        { modal_substance_filter() }
        { craft_item_modal() }
    }
}

fn dc_slider() -> impl IntoView {
    let val = move || expect_rw::<CraftState>().with(|state| state.dc_points());

    view! {
        <div class= "relative w-full">
            <input
                class= "dc-score-slider"
                type= "range"
                disabled=true
                value=val
                min=0
                max=59
            />
            <div class= "dc-score-markers">
              <span /><span /><span /><span />
            </div>
        </div>
    }
}
