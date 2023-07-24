use std::cmp;

use leptos::*;
use leptos_router::*;

mod hp;

use hp::*;

use super::{EquipSlot, PCSession, PC};
use crate::items::enhancement::Feature;
use crate::items::Held;
use crate::pc::PCStat;
use crate::svg;
use crate::utils::read_context;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    let pc = read_context::<PC>(cx);
    let sesh = read_context::<PCSession>(cx);

    view! {
        cx,
        <div class= "flex flex-col space-y-2 px-2">
            <h2> {move || pc.with(|p| p.name.clone())} </h2>
            <AbilityScores />
            { move || sesh.with(|sesh| substats(cx, sesh)) }
            <HP />
            <div class= "">
                <h3> "Description" </h3>
                <div class= "italic">
                    { move || pc.with(|p| p.description.clone()) }
                </div>
            </div>
            <div class= "">
                <h3> "Features" </h3>
                { move || pc.with(|pc| {
                    sesh.with_untracked(|sesh| weapon_attack(cx, pc, sesh))
                })}
                { move || sesh.with(|sesh| features(cx, sesh)) }
            </div>
            <div>
                <A href="/" class= "flex-centered p-2 mt-8 bg-red-900 rounded">
                    <div class= "w-6 mr-2 svg" inner_html=svg::HOME />
                    <div class= "mt-1"> "Lobby" </div>
                </A>
            </div>
            <div class= "psuedo h-4" />
        </div>
    }
}

/// Gives the damage die to be used for a given `power`.
/// Defaults to maximum die size if `max` AND `scale` is too large (or small).
fn damage_die(scale: i32, max: i32) -> String {
    const DAMAGE_DICE: [&str; 10] = [
        "1", "1d4", "1d6", "1d8", "1d10", "1d12", "2d6", "2d8", "2d10", "2d12",
    ];
    let i = usize::try_from(cmp::min(scale, max)).unwrap_or(0);
    DAMAGE_DICE.get(i).unwrap_or(&DAMAGE_DICE[9]).to_string()
}

fn weapon_attack(cx: Scope, pc: &PC, sesh: &PCSession) -> impl IntoView {
    let (name, dmg) = if let Some(item) = &pc.equipment[EquipSlot::MainHand.index()] {
        let item = item.as_held().unwrap();
        let scale_stat = &sesh.stats[item.base.scale_by().index()];
        let max_dmg = &sesh.stats[PCStat::Might.index()];
        let dmg = damage_die(*scale_stat, *max_dmg);
        let dual_wiedling = pc.equipment.as_ref()[EquipSlot::OffHand.index()]
            .clone()
            .and_then(|item| match item.as_held()?.base {
                Held::Sword | Held::Dagger | Held::Warhammer | Held::Handaxe => {
                    Some("Flurry of Blows")
                }
                _ => None,
            });
        if let Some(name) = dual_wiedling {
            (name.to_string(), dmg)
        } else {
            let name = match item.base {
                Held::Sword => "Slash",
                Held::Dagger => "Hidden Blade",
                Held::Crowsbeak | Held::Warhammer => "Crushing Blow",
                Held::Handaxe => "Chop",
                Held::Bow | Held::Crossbow => "Marksman's Fang",
                Held::Shield => "Shield Bash",
                _ => "Improvised Bash",
            };
            (name.to_string(), dmg)
        }
    } else {
        ("Unarmed Strike".into(), "1".into())
    };

    view! {
        cx,
        <div class= "bg-zinc-700 rounded flex-col flex mt-2">
            <div class= "italic text-sm text-center"> "Primary Attack" </div>
            <div class= "flex items-center justify-around px-2 py-1">
                <div> { name } </div>
                <div class= "rounded flex-centered bg-sky-700 px-2 h-8"> { dmg } </div>
            </div>
        </div>
    }
}

fn features(cx: Scope, sesh: &PCSession) -> View {
    sesh.features
        .iter()
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

fn substats(cx: Scope, sesh: &PCSession) -> impl IntoView {
    let speed = &sesh.stats[PCStat::Speed.index()];
    let sorcery = &sesh.stats[PCStat::Sorcery.index()];
    let might = &sesh.stats[PCStat::Might.index()];
    let might_dice = damage_die(*might, *might);
    const BOX: &str = "bg-teal-800 rounded";
    const TEXT: &str = "italic text-sm";
    view! {
        cx,
        <div class= "grid grid-cols-3 text-center gap-2">
            <div class=BOX >
                <div class=TEXT> "Speed" </div>
                <div> { format!("{speed} ft.") } </div>
            </div>
            <div class=BOX>
                <div class=TEXT> "Might" </div>
                <div> { format!("{might} ({might_dice})") } </div>
            </div>
            <div class=BOX>
                <div class=TEXT> "Sorcery" </div>
                <div> {*sorcery}</div>
            </div>
        </div>
    }
}

#[component]
fn Ability(cx: Scope, abi: PCStat) -> impl IntoView {
    let score = move || {
        read_context::<PCSession>(cx).with(|s| {
            let num = s.stats[abi.index()];
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
