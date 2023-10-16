use leptos::*;
use leptos_router::*;

use self::hp::hp;
use self::prof::prof_view;
use self::quick_access::quick_access;
use self::turn_tracker::turn_tracker;
use crate::icons;
use crate::pc::session::PCSession;
use crate::pc::{PCStat, PC};
use crate::utils::{split_operator, RwProvided};

mod buff_list;
mod buff_search;
mod buff_view;
mod hp;
mod prof;
mod quick_access;
mod turn_tracker;

pub fn overview() -> impl IntoView {
    let name = PC::with(|pc| {
        view! {
            <h3 class= "my-2 w-12 grow line-clamp-2"> { &pc.name } </h3>
        }
        .into_view()
    });

    view! {
        <div class= "flex items-center">
            { name }
            <A href= "/" class= "btn p-2 bg-red-800 aspect-square flex-center">
                <div class= "w-5 -mr-1" inner_html=icons::EXIT />
            </A>
        </div>
        { ability_scores }
        <div class= "grid grid-cols-7 gap-y-1 gap-x-2">
            { class_view }
            { prof_view }
            { hp }
        </div>
        <h4 class= "text-center"> "Turn Tracker" </h4>
        { turn_tracker }
        <h4 class= "text-center"> "Quick Access" </h4>
        { quick_access }
        <h4 class= "text-center"> "Buffs & Debuffs" </h4>
        { buff_search::search  }
        { buff_list::list }
    }
}

fn ability_scores() -> impl IntoView {
    const CORE_STATS: [PCStat; 4] = [PCStat::STR, PCStat::DEX, PCStat::INT, PCStat::CHA];
    let names = CORE_STATS
        .map(|x| {
            view! { <div> { x.to_string() } </div> }
        })
        .collect_view();
    let stats = move || {
        PCSession::with(|sesh| {
            CORE_STATS
                .map(|x| {
                    let (op, stat) = split_operator(sesh.stats.get(x));
                    view! { <div> { format!("{op}{stat}") } </div> }
                })
                .collect_view()
        })
    };

    view! {
        <div class= "grid grid-cols-4 divide-x-2 divide-rm-5 divide-sky-700 font-tight text-2xl text-center">
            { names }
            { stats }
        </div>
    }
}

fn class_view() -> impl IntoView {
    let text = PC::with(|pc| {
        let (class, level) = pc.class;
        format!("{class} LEVEL {level}")
    });

    view! {
        <A class= "btn bg-surface flex-center py-2" href= "../class">
            <div class= "w-6 fill-yellow-500" inner_html=icons::BULLSEYE />
        </A>
        <div class= "col-span-6 uppercase font-tight self-center text-xl">
            { text }
        </div>
    }
}
