use leptos::*;

mod hp;

use hp::*;

use super::{EquipSlot, PCSession, PC};
use crate::items::enhancement::Feature;
use crate::pc::PCStat;
use crate::utils::read_context;

fn weap_feat(pc: &PC) -> Feature {
    if let Some(_) = pc.get_equip(EquipSlot::MainHand) {
        Feature::new("Flurry of blows", "Deal 1d6 dmg", 0)
    } else {
        Feature::new("Unarmed Strike", "Deal 1 dmg", 0)
    }
}

fn features(cx: Scope, sesh: &PCSession, armed: Feature) -> View {
    let mut arr = vec![armed];
    arr.extend_from_slice(&sesh.features);
    arr.into_iter()
        .map(|Feature { name, effect, uses }| {
            view! {
                cx,
                <div>
                    <div> { name } </div>
                    <div> { effect } </div>
                    <div> { uses.to_string() } </div>
                </div>
            }
        })
        .collect_view(cx)
}

#[component]
pub fn Basics(cx: Scope) -> impl IntoView {
    let pc = read_context::<PC>(cx);

    view! {
        cx,
        <div class= "flex flex-col space-y-2 px-2">
            <h2> {move || pc.with(|p| p.name.clone())} </h2>
            <div class= "italic">
                { move || pc.with(|p| p.description.clone()) }
            </div>
            <AbilityScores />
            <HP />
            <div class= "">
                <h3> "Conditions" </h3>
            </div>
            <div class= "">
                <h3> "Features" </h3>
                { move || pc.with(|pc| {
                    let sesh = read_context::<PCSession>(cx);
                    sesh.with_untracked(|sesh| features(cx, sesh, weap_feat(pc)))
                })}
            </div>
        </div>
    }
}

#[component]
fn Ability(cx: Scope, abi: PCStat) -> impl IntoView {
    let score = move || {
        read_context::<PCSession>(cx).with(|s| {
            let num = *s.stats.get(abi);
            if num > -1 {
                format!("+{num}")
            } else {
                num.to_string()
            }
        })
    };

    view! {
        cx,
        <div class= "bg-emerald-900 rounded-xl text-center">
            <div class= "">
                { abi.to_string() }
            </div>
            <h4>
                { move || score() }
            </h4>
        </div>
    }
}

#[component]
fn AbilityScores(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class= "grid grid-cols-4 grid-rows-1 grid-flow-col gap-2">
            <Ability abi=PCStat::STR />
            <Ability abi=PCStat::DEX />
            <Ability abi=PCStat::INT />
            <Ability abi=PCStat::CHA />
        </div>
    }
}
