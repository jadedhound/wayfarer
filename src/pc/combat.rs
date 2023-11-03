use leptos::*;
use leptos_router::*;

use self::hp::hp;
use self::prof::prof_view;
use self::quick_access::quick_access;
use self::time_tracker::time_tracker;
use crate::icons;
use crate::lobby::PCList;
use crate::pc::session::Session;
use crate::pc::{Ability, PC};
use crate::utils::add_operator;
use crate::utils::rw_utils::RwUtils;

mod buff_list;
mod buff_search;
mod buff_view;
mod hp;
mod prof;
mod quick_access;
mod time_tracker;
mod use_button;

pub fn combat() -> impl IntoView {
    view! {
        { name }
        { ability_scores }
        <div class= "grid grid-cols-7 gap-y-1 gap-x-2">
            { class_view }
            { prof_view }
            { hp }
        </div>
        <h4 class= "text-center"> "Time Tracker" </h4>
        { time_tracker }
        <h4 class= "text-center"> "Quick Access" </h4>
        { quick_access }
        <h4 class= "text-center"> "Buffs & Debuffs" </h4>
        { buff_search::search  }
        { buff_list::list }
        <A href= "/" class= "btn bg-red-800 flex-center gap-2 mt-4">
            <div class= "w-5" inner_html=icons::EXIT />
            "EXIT TO LOBBY"
        </A>
    }
}

fn name() -> impl IntoView {
    let (pc_list, pc) = (PCList::expect(), PC::expect());
    let id = use_location().pathname.with_untracked(|path| {
        path.split('/')
            .nth(2)
            .and_then(|id_str| id_str.parse::<usize>().ok())
            .unwrap()
    });
    let editting = RwSignal::new(false);
    let name = RwSignal::new(pc.with_untracked(|pc| pc.name.clone()));
    let disabled = move || name.with(|name| name.is_empty()) && editting.get();
    let edit_or_save = move |_| {
        if editting.get() {
            pc.update(|pc| pc.name = name.get());
            pc_list.update(|list| {
                if let Some(basic) = list.0.get_mut(id) {
                    basic.name = name.get();
                }
            })
        }
        editting.update(|edit| *edit = !*edit);
    };
    let colour = move || {
        if editting.get() {
            "bg-green-800"
        } else {
            "bg-surface"
        }
    };

    view! {
        <div class= "flex gap-2">
            <button
                class=move || format!("btn {}", colour())
                on:click=edit_or_save
                disabled=disabled
            >
                <div class= "w-5" hidden=editting inner_html=icons::QUILL />
                <div class= "w-5" hidden=move || !editting.get() inner_html=icons::CHECKMARK />
            </button>
            <h3 class= "my-2 w-12 grow line-clamp-2" hidden=editting>
                { name }
            </h3>
            <input
                class= "input w-12 grow text-4xl font-regal"
                on:input=move |ev| name.set(event_target_value(&ev))
                prop:value=name
                hidden=move || !editting.get()
                maxlength=30
            />
        </div>
    }
}

fn ability_scores() -> impl IntoView {
    const CORE_STATS: [Ability; 4] = [Ability::STR, Ability::DEX, Ability::INT, Ability::CHA];
    let sesh = Session::expect();
    let names = CORE_STATS
        .map(|x| {
            view! { <div> { x.to_string() } </div> }
        })
        .collect_view();
    let stats = move || {
        sesh.with(|sesh| {
            CORE_STATS
                .map(|x| {
                    let score = add_operator(sesh.abi_scores.get(x));
                    view! { <div> { score } </div> }
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
    let text = PC::expect().with(|pc| {
        let (class, exp) = pc.class;
        format!("{class} LEVEL {}", exp.level().get())
    });

    view! {
        <A class= "btn bg-surface flex-center" href= "../class">
            <div class= "w-6 fill-yellow-500" inner_html=icons::BULLSEYE />
        </A>
        <div class= "col-span-6 uppercase font-tight self-center text-xl">
            { text }
        </div>
    }
}
