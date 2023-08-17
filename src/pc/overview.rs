use leptos::*;
use leptos_router::*;

mod buffs;
mod hp;
mod quick_access;
mod rest;
mod tome;
mod turn_tracker;
use self::turn_tracker::turn_tracker;
use super::PC;
use crate::pc::overview::buffs::buff_listview;
use crate::pc::overview::hp::hp;
use crate::pc::overview::quick_access::quick_access;
use crate::pc::overview::rest::rest;
use crate::pc::session::PCSession;
use crate::pc::PCStat;
use crate::svg;
use crate::utils::{expect_rw, split_operator, RwProvided};

pub fn overview() -> impl IntoView {
    let pc = expect_rw::<PC>();

    view! {
        <div class= "flex flex-col space-y-2 px-2">
            <h2> {move || pc.with(|p| p.name.clone())} </h2>
            { ability_scores }
            { hp }
            <h5 class= "text-center"> "TURN TRACKER" </h5>
            { turn_tracker }
            <h5 class= "text-center"> "QUICK ACCESS" </h5>
            { quick_access }
            <h5 class= "text-center"> "BUFFS AND DEBUFFS" </h5>
            { buff_listview }
            <h5 class= "text-center"> "DESCRIPTION" </h5>
            { move || pc.with(|p| p.description.clone()) }
            <h5 class= "text-center"> "REST" </h5>
            { rest }
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

fn ability_scores() -> impl IntoView {
    view! {

        <div class= "grid grid-cols-4 gap-2 divide-x-2 divide-emerald-700">
            { ability( PCStat::STR) }
            { ability( PCStat::DEX) }
            { ability( PCStat::INT) }
            { ability( PCStat::CHA) }
        </div>
    }
}

fn ability(stat: PCStat) -> impl IntoView {
    let score = move || PCSession::with(|sesh| split_operator(sesh.stats.get(stat)));

    view! {
        <div class= "text-center font-sans text-2xl">
            <div> { stat.to_string() } </div>
            <div> { score } </div>
        </div>
    }
}
