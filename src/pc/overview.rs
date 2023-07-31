use leptos::*;
use leptos_router::*;

mod buffs;
mod hp;
mod primary;
mod tome;
use super::PC;
use crate::items::features::Feature;
use crate::pc::overview::buffs::BuffListview;
use crate::pc::overview::hp::HP;
use crate::pc::overview::primary::Primary;
use crate::pc::session::PCSession;
use crate::pc::PCStat;
use crate::svg;
use crate::utils::rw_context;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    let pc = rw_context::<PC>(cx);
    let sesh = rw_context::<PCSession>(cx);

    view! {
        cx,
        <div class= "flex flex-col space-y-2 px-2">
            <h2> {move || pc.with(|p| p.name.clone())} </h2>
            <AbilityScores />
            <hr class= "border-emerald-700 border-1 bg-emerald-700" />
            <SubStats />
            <HP />
            <BuffListview />
            <div class= "">
                <h3> "Features" </h3>
                <Primary />
                { move || sesh.with(|sesh| features(cx, sesh)) }
            </div>
            <div class= "">
                <h3> "Description" </h3>
                <div class= "italic">
                    { move || pc.with(|p| p.description.clone()) }
                </div>
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

fn features(cx: Scope, sesh: &PCSession) -> View {
    sesh.features
        .iter()
        .map(|Feature { name, effect }| {
            view! {
                cx,
                <div>
                    <div> { name } </div>
                    <div> { effect } </div>
                </div>
            }
        })
        .collect_view(cx)
}

#[component]
fn SubStats(cx: Scope) -> impl IntoView {
    let sesh = rw_context::<PCSession>(cx);
    let speed = move || sesh.with(|sesh| format!("{} ft.", sesh.stats[PCStat::Speed.index()]));
    let sorc = move || sesh.with(|sesh| sesh.stats[PCStat::Sorcery.index()].to_string());
    const TEXT: &str = "italic text-sm";
    view! {
        cx,
        <div class= "grid grid-cols-2 text-center gap-2 divide-x-2 divide-emerald-700 font-sans">
            <div>
                <div class=TEXT> "Speed" </div>
                <div> { speed } </div>
            </div>
            <div>
                <div class=TEXT> "Sorcery" </div>
                <div> { sorc } </div>
            </div>
        </div>
    }
}

#[component]
fn Ability(cx: Scope, abi: PCStat) -> impl IntoView {
    let score = move || {
        rw_context::<PCSession>(cx).with(|s| {
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
        <div class= "text-center font-sans">
            <div class= "">
                { abi.to_string() }
            </div>
            <h4>
                { score }
            </h4>
        </div>
    }
}

#[component]
fn AbilityScores(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class= "grid grid-cols-4 gap-2 divide-x-2 divide-emerald-700">
            <Ability abi=PCStat::STR />
            <Ability abi=PCStat::DEX />
            <Ability abi=PCStat::INT />
            <Ability abi=PCStat::CHA />
        </div>
    }
}
