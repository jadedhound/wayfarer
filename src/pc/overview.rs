use leptos::*;
use leptos_router::*;

mod buffs;
mod hp;
mod quick_access;
mod rest;
mod tome;
use super::PC;
use crate::pc::overview::buffs::BuffListview;
use crate::pc::overview::hp::HP;
use crate::pc::overview::quick_access::QuickAccess;
use crate::pc::overview::rest::Rest;
use crate::pc::session::PCSession;
use crate::pc::PCStat;
use crate::svg;
use crate::utils::{rw_context, split_operator};

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    let pc = rw_context::<PC>(cx);

    view! {
        cx,
        <div class= "flex flex-col space-y-2 px-2">
            <h2> {move || pc.with(|p| p.name.clone())} </h2>
            <AbilityScores />
            <HP />
            <h5 class= "text-center"> "QUICK ACCESS" </h5>
            <QuickAccess />
            <h5 class= "text-center"> "BUFFS AND DEBUFFS" </h5>
            <BuffListview />
            <h5 class= "text-center"> "DESCRIPTION" </h5>
            { move || pc.with(|p| p.description.clone()) }
            <h5 class= "text-center"> "REST" </h5>
            <Rest />
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

#[component]
fn Ability(cx: Scope, stat: PCStat) -> impl IntoView {
    let score = move || rw_context::<PCSession>(cx).with(|s| split_operator(s.stats[stat.index()]));

    view! {
        cx,
        <div class= "text-center font-sans text-2xl">
            <div> { stat.to_string() } </div>
            <div> { score } </div>
        </div>
    }
}

#[component]
fn AbilityScores(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class= "grid grid-cols-4 gap-2 divide-x-2 divide-emerald-700">
            <Ability stat=PCStat::STR />
            <Ability stat=PCStat::DEX />
            <Ability stat=PCStat::INT />
            <Ability stat=PCStat::CHA />
        </div>
    }
}
