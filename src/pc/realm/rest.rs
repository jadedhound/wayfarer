use leptos::*;
use strum::Display;

use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::utils::turns::{Turns, TURNS_IN_DAY};
use crate::views::checkbox::Checkbox;
use crate::views::revealer::{RevLocation, Revealer};
use crate::views::toast::Toast;

#[derive(Clone, Copy, Default, Debug)]
struct State {
    firewood: Outcome,
    cooking: Outcome,
    camaraderie: Outcome,
    has_tent: bool,
    has_bedroll: bool,
}

#[derive(Default, Display, Clone, Copy, PartialEq, Debug)]
enum Outcome {
    #[default]
    Failure,
    #[strum(serialize = "Partial Success")]
    PartialSuccess,
    Success,
}

pub fn rest() -> impl IntoView {
    let pc = PC::expect();
    let state = State::provide();
    let (has_bedroll, has_tent) = pc.with_untracked(|pc| {
        let has_bedroll = pc
            .inventory
            .values()
            .any(|item| item.name.contains("bedroll"));
        let has_tent = pc.inventory.values().any(|item| item.name.contains("tent"));
        (has_bedroll, has_tent)
    });
    state.update(|state| {
        state.has_bedroll = has_bedroll;
        state.has_tent = has_tent;
    });

    view! {
        <div class= "italic text-center">
            "Split the group to accomplish the following tasks; sleep tonight might be
            the only thing that stands between you and a grizzly fate tomorrow."
        </div>
        <div class= "grid grid-cols-3 gap-1">
            <h6 class= "text-center col-span-2 col-span-3"> "Firewood" </h6>
            <div class= "italic text-center col-span-3">
                "Gather 1d6 hours of firewood, modified by weather conditions."
            </div>
            { outcome_grid(|state| state.firewood, |state, value| state.firewood = value) }
            <h6 class= "text-center col-span-2 col-span-3"> "Cooking" </h6>
            <div class= "italic text-center col-span-3">
                "Expend rations and a INT check to cook a hearty meal for all."
            </div>
            { outcome_grid(|state| state.cooking, |state, value| state.cooking = value) }
            <h6 class= "text-center col-span-2 col-span-3"> "Camaraderie" </h6>
            <div class= "italic text-center col-span-3">
                "CHA check to lighten the mood and comfort your fellow adventurers."
            </div>
            { outcome_grid(|state| state.camaraderie, |state, value| state.camaraderie = value) }
        </div>
        <div class= "grid grid-cols-2 gap-1">
            <h6 class= "text-center col-span-2"> "Equipment" </h6>
            { equipment(
                "BEDROLL",
                |state| state.has_bedroll,
                |state| state.has_bedroll = !state.has_bedroll
            ) }
            { equipment(
                "TENT",
                |state| state.has_tent,
                |state| state.has_tent = !state.has_tent
            ) }
            <h6 class= "text-center col-span-2"> "Seasons" </h6>
            { seasons }
        </div>
        <h6 class= "text-center"> "Result" </h6>
        <div>
            <span class= "font-tight text-red-500"> "FAILURE. " </span>
            "+1 fatigue."
            <br />
            <span class= "font-tight text-orange-500"> "PARTIAL. " </span>
            "-1 fatigue, guard restored, buffs recharged."
            <br />
            <span class= "font-tight text-green-500"> "SUCCESS. " </span>
            "-1 fatigue, guard restored, buffs recharged, +1 health."
        </div>
        <div class= "grid grid-cols-3 gap-1">
            { complete_rest }
        </div>
    }
}

fn outcome_grid<G, S>(getter: G, setter: S) -> impl IntoView
where
    G: Fn(&State) -> Outcome + Copy + 'static,
    S: Fn(&mut State, Outcome) + Copy + 'static,
{
    let state = State::expect();
    let outcome_button = move |outcome: Outcome| {
        let change_outcome = move || state.update(|state| setter(state, outcome));
        let checked = State::slice(move |state| getter(state) == outcome);
        view! {
            <Checkbox
                checked
                on_click=change_outcome
                checked_colour= "border-yellow-500 text-yellow-400"
                class= "p-2"
            >
                { outcome.to_string() }
            </Checkbox>
        }
    };
    view! {
        { outcome_button(Outcome::Success) }
        { outcome_button(Outcome::PartialSuccess) }
        { outcome_button(Outcome::Failure) }
    }
}

fn equipment<F, C>(name: &'static str, equipment: F, change_value: C) -> impl IntoView
where
    F: Fn(&State) -> bool + Copy + 'static,
    C: Fn(&mut State) + Copy + 'static,
{
    let state = State::expect();
    let has_equipment = State::slice(equipment);
    let change_value = move || state.update(change_value);
    view! {
        <Checkbox
            checked=has_equipment
            on_click=change_value
            checked_colour= "border-yellow-500 text-yellow-400"
            class= "p-2"
        >
            { name }
        </Checkbox>
    }
}

fn seasons() -> impl IntoView {
    let outcomes = State::slice(|state| {
        state.firewood as u8
            + state.cooking as u8
            + state.camaraderie as u8
            + state.has_tent as u8
            + state.has_bedroll as u8
    });
    let season = move |name: &'static str, dc: u8, class: &'static str, icon: &'static str| {
        let dc = move || dc.saturating_sub(outcomes.get());
        view! {
            <div class=format!("flex justify-center font-tight gap-2 {class}")>
                <div class= "w-5" inner_html=icon />
                <div> { format!("{name} DC {}", dc())} </div>
            </div>
        }
    };
    view! {
        { season("WINTER", 18, "text-sky-500 fill-sky-500", icons::SNOWFLAKE) }
        { season("SUMMER", 8, "text-amber-400 fill-amber-400", icons::SUN) }
        { season("SPRING/FALL", 13, "text-lime-500 fill-lime-500 col-span-2", icons::LEAF) }
    }
}

fn complete_rest() -> impl IntoView {
    fn action_button<F>(name: &'static str, class: &'static str, onclick: F) -> impl IntoView
    where
        F: Fn() + Copy + 'static,
    {
        let pc = PC::expect();
        let onclick = move |_| {
            onclick();
            pc.update(|pc| pc.turns.add(TURNS_IN_DAY));
            Revealer::hide();
        };
        view! {
            <button
                class=format!("btn z-40 {class}")
                on:click=onclick
                hidden=move || Revealer::is_hidden(RevLocation::RestConfirm, 0)
            >
                { name }
            </button>
        }
    }

    let pc = PC::expect();
    let failure = move || {
        pc.update(|pc| pc.fatigue += 1);
        Toast::show("rest failure", "fatigue added to inventory");
    };
    let partial = move || {
        pc.update(|pc| {
            pc.guard_dmg = 0;
            pc.fatigue = pc.fatigue.saturating_sub(1);
        });
        Toast::show("partial rest", "fatigue removed and guard restored");
    };
    let success = move || {
        pc.update(|pc| {
            pc.guard_dmg = 0;
            pc.health_dmg = pc.health_dmg.saturating_sub(1);
            pc.fatigue = pc.fatigue.saturating_sub(1);
        });
        Toast::show(
            "rest success",
            "fatigue removed, guard restored and +1 health",
        );
    };

    view! {
        <button
            class= "btn bg-surface col-span-3"
            on:click=move |_| Revealer::show(RevLocation::RestConfirm, 0)
            hidden=move || Revealer::is_shown(RevLocation::RestConfirm, 0)
        >
            "CONFIRM"
        </button>
        { action_button("SUCCESS", "bg-green-800", success) }
        { action_button("PARTIAL", "bg-orange-700", partial) }
        { action_button("FAILURE", "bg-red-800", failure) }
    }
}

// -----------------------------------
// TRAIT IMPL
// -----------------------------------

impl RwUtils for State {}
