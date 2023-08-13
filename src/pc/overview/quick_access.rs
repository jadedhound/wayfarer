use leptos::*;

use crate::items::effects::Effect;
use crate::items::item_spec::ItemSpec;
use crate::items::weapons::{Weapon, DAMAGE_DIE};
use crate::items::Item;
use crate::pc::overview::tome::TomeView;
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::utils::{capitalise, rw_context, split_operator};

#[component]
pub(super) fn QuickAccess(cx: Scope) -> impl IntoView {
    let quick_access = move || {
        rw_context::<PC>(cx).with(|pc| {
            let mut quick_access = pc.quick_access.iter().enumerate().flat_map(|(id, item)| {
                let item = item.as_ref()?;
                Some(spec_to_view(cx, id, item))
            });
            // Show help text if quick_access is empty.
            if let Some(slot_1) = quick_access.next() {
                view! { cx,
                    { slot_1 }
                    { quick_access.collect_view(cx) }
                }
                .into_view(cx)
            } else {
                view! { cx, <HelpText /> }.into_view(cx)
            }
        })
    };
    view! { cx,
        <div class= "flex flex-col gap-y-2">
            { quick_access }
        </div>
    }
}

fn spec_to_view(cx: Scope, id: usize, item: &Item) -> impl IntoView {
    let name = item.name.to_uppercase();
    match &item.spec {
        ItemSpec::Tome(tome) => view! { cx, <TomeView item=item tome /> }.into_view(cx),
        ItemSpec::Consumable(effect) => view! { cx, <UseItem name id effect /> }.into_view(cx),
        _ => view! { cx, <BasicItem name /> }.into_view(cx),
    }
}

#[component]
fn HelpText(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class= "text-center">
            "Items in quick access slots will be shown here and can be used as an action."
        </div>
    }
}

#[component]
fn BasicItem(cx: Scope, name: String) -> impl IntoView {
    view! { cx,
        <div class= "h-12 border-2 border-zinc-700 rounded flex items-center px-2">
            { name }
        </div>
    }
}

#[component]
fn NameAndDmg(cx: Scope, name: String, dmg: String) -> impl IntoView {
    view! { cx,
        <div class= " rounded border-2 border-zinc-700 flex items-center font-sans">
            <div class= "ml-2 py-2 w-12 grow"> { capitalise(&name) } </div>
            <div class= "border-l-2 border-zinc-700 px-4 w-28 text-center">
                { dmg }
            </div>
        </div>
    }
}

#[component]
fn WeaponAtk(cx: Scope, name: String, weap: Weapon) -> impl IntoView {
    let sesh = rw_context::<PCSession>(cx);
    let dmg_incr = move || {
        let (op, num) = split_operator(sesh.with(|sesh| sesh.stats[weap.as_stat().index()]));
        format!("{op}{num}")
    };
    let name = format!("{} attack", name);
    let dmg = format!("{}{}", DAMAGE_DIE[weap.as_damage()], dmg_incr());
    view! { cx,
        <NameAndDmg name dmg />
    }
}

fn use_item(cx: Scope, id: usize) {
    rw_context::<PC>(cx).update(|pc| {
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

#[component]
fn UseItem<'a>(cx: Scope, name: String, effect: &'a Effect, id: usize) -> impl IntoView {
    let uses_left = move || {
        rw_context::<PC>(cx).with(|pc| {
            pc.quick_access[id]
                .as_ref()
                .map(|item| {
                    let stacks = item.stacks.map(|(curr, _)| curr).unwrap_or(1);
                    format!("{stacks} uses left.")
                })
                .unwrap_or_default()
        })
    };
    view! { cx,
        <div class= "flex">
            <div class= "flex flex-col rounded-l border-y-2 border-l-2 border-zinc-700 px-2 w-full">
                { name }
                { effect.to_string() }
                <span class= "font-sans text-center"> { uses_left } </span>
            </div>
            <button
                class= "rounded-r bg-red-800 flex-centered w-12 font-sans"
                on:click=move |_| use_item(cx, id)
            >
                "USE"
            </button>
        </div>
    }
}
