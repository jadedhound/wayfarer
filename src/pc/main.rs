use leptos::*;
use leptos_router::*;

use crate::icons;
use crate::lobby::PCList;
use crate::pc::session::Session;
use crate::pc::{Ability, PC};
use crate::utils::rw_utils::RwUtils;
use crate::utils::{add_operator, concat_if};
use crate::views::inventory::inventory_view;

mod fatigue;
mod hp;
mod prof;
mod recently_removed;
mod search;
mod turn_tracker;
mod use_button;
mod wealth;

pub fn main() -> impl IntoView {
    let spacer_hidden = PC::slice(|pc| pc.backpack.vacancy().is_some_and(|vacancy| vacancy > 0));

    view! {
        { name }
        { ability_scores }
        <div class= "grid grid-cols-7 gap-y-1 gap-x-2">
            { prof::prof_view }
            { hp::hp }
        </div>
        { turn_tracker::turn_tracker }
        <h4 class= "text-center"> "Equipment" </h4>
        { inventory_view(|pc| &pc.equipment, |pc| &mut pc.equipment, equipment_options) }
        { wealth::wealth }
        <h4 class= "text-center"> "Backpack" </h4>
        { search::search_view }
        { fatigue::fatigue }
        { inventory_view(|pc| &pc.backpack, |pc| &mut pc.backpack, backpack_options) }
        { recently_removed::button }
        <div class= "psuedo h-16" hidden=spacer_hidden />
        { recently_removed::modal }
    }
}

fn backpack_options(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let move_item = move |_| {
        pc.update(|pc| {
            if let Some(item) = pc.backpack.remove(id) {
                pc.equipment.add(item);
            }
        })
    };
    let cannot_equip = PC::slice(|pc| !pc.equipment.vacancy().is_some_and(|amount| amount > 0));
    view! {
        <button hidden=cannot_equip on:click=move_item> "EQUIP" </button>
    }
}

fn equipment_options(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let move_item = move |_| {
        pc.update(|pc| {
            if let Some(item) = pc.equipment.remove(id) {
                pc.backpack.add(item);
            }
        })
    };
    view! {
        <button on:click=move_item> "UNEQUIP" </button>
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

    view! {
        <div class= "flex gap-2">
            <button
                class=concat_if(editting.into(), "btn", "bg-green-800", "bg-surface")
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
