use leptos::*;

use crate::items::effects::Effect;
use crate::items::item_spec::ItemSpec;
use crate::items::weapons::{Weapon, DAMAGE_DIE};
use crate::items::Item;
use crate::pc::overview::tome::tome_view;
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::utils::{expect_rw, split_operator};

pub(super) fn quick_access() -> impl IntoView {
    move || {
        expect_rw::<PC>().with(|pc| {
            let mut quick_access = pc.quick_access.iter().enumerate().flat_map(|(id, item)| {
                let item = item.as_ref()?;
                Some(spec_to_view(id, item))
            });
            // Show help text if quick_access is empty.
            if let Some(slot_1) = quick_access.next() {
                view! {
                    <div class= "flex flex-col shaded-table">
                        { slot_1 }
                        { quick_access.collect_view() }
                    </div>
                }
                .into_view()
            } else {
                help_text().into_view()
            }
        })
    }
}

fn spec_to_view(id: usize, item: &Item) -> impl IntoView {
    let name = item.name.clone();
    match &item.spec {
        ItemSpec::Weapon(weapon) => weapon_attack(name, weapon).into_view(),
        ItemSpec::Tome(tome) => tome_view(item, tome).into_view(),
        ItemSpec::Consumable(effect) => consumable_view(item.name.clone(), effect, id).into_view(),
        _ => basic_item(item.name.clone()).into_view(),
    }
}

fn help_text() -> impl IntoView {
    view! {
        <div class= "text-center italic p-2">
            "Items in quick access slots will be shown here."
        </div>
    }
}

fn basic_item(name: String) -> impl IntoView {
    view! {
        <div class= "p-2 uppercase title">
            { name }
        </div>
    }
}

fn weapon_attack(name: String, weap: &Weapon) -> impl IntoView {
    let sesh = expect_rw::<PCSession>();
    let dmg_incr = move || {
        let (op, num) = split_operator(sesh.with(|sesh| sesh.stats.get(weap.as_stat())));
        format!("{op}{num}")
    };
    let dmg = format!("{}{}", DAMAGE_DIE[weap.as_damage()], dmg_incr());

    view! {
        <div class= "flex items-center">
            <div class= "p-2 w-12 grow uppercase title"> { name } </div>
            <div class= "w-16 text-center font-sans">
                { dmg }
            </div>
        </div>
    }
}

fn use_item(id: usize) {
    expect_rw::<PC>().update(|pc| {
        let item = pc.quick_access[id].as_mut().unwrap();
        if let Some(x) = item.stacks.as_mut() {
            if x.0 > 1 {
                x.0 -= 1;
            } else {
                pc.quick_access[id] = None;
            }
        } else {
            pc.quick_access[id] = None;
        }
    })
}

fn consumable_view(name: String, effect: &Effect, id: usize) -> impl IntoView {
    let uses_left = move || {
        expect_rw::<PC>().with(|pc| {
            pc.quick_access[id]
                .as_ref()
                .map(|item| {
                    let stacks = item.stacks.map(|(curr, _)| curr).unwrap_or(1);
                    let uses = if stacks > 2 { "uses" } else { "use" };
                    format!("{stacks} {uses} left.")
                })
                .unwrap_or_default()
        })
    };
    let effect = format!("Action: {effect}.");

    view! {
        <div class= "flex">
            <div class= "flex flex-col p-2 w-12 grow">
                <div class= "uppercase title"> { name } </div>
                <div> { effect } </div>
                <div> { uses_left } </div>
            </div>
            <button
                class= "rounded-r flex-centered w-16 font-sans text-emerald-500"
                on:click=move |_| use_item( id)
            >
                "USE"
            </button>
        </div>
    }
}
