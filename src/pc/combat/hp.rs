use std::cmp;

use leptos::*;
use web_sys::Event;

use crate::icons;
use crate::pc::session::Session;
use crate::pc::{Ability, PC};
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;

#[derive(Clone, Copy, Default)]
struct State {
    is_raw: bool,
    value: u32,
}

impl RwUtils for State {}

pub fn hp() -> impl IntoView {
    State::provide();

    view! {
        { guard }
        { health }
        { heal_button }
        { user_input }
        { damage_button }
    }
}

fn guard() -> impl IntoView {
    let pc = PC::expect();
    let max = Session::slice(|sesh| sesh.abi_scores.get(Ability::Guard) as u32);
    let curr = move || max.get().saturating_sub(pc.with(|pc| pc.guard_dmg));
    let num_or_icon = move || {
        const COLOUR: &str = "fill-green-800";
        let curr = curr();
        if curr > 0 {
            view! {
                <div class=COLOUR  inner_html=icons::SHIELD />
                <h5 class= "absolute text-center top-0 w-full pt-1"> { curr } </h5>
            }
            .into_view()
        } else {
            view! {
                <div class=COLOUR inner_html=icons::SHIELD_BROKEN />
            }
            .into_view()
        }
    };

    view! {
        <div class= "relative">
            { num_or_icon }
        </div>
        <input
            class= "range green-bar col-span-6 bg-inherit border-2 border-stone-400 pointer-events-none"
            type= "range"
            min=0
            max=max
            value=curr
        />
    }
}

fn health() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let is_not_raw = create_read_slice(state, move |state| !state.is_raw);
    let invert_raw = move || state.update(|x| x.is_raw = !x.is_raw);
    let max = Session::slice(|sesh| sesh.abi_scores.get(Ability::Health) as u32);
    let curr = move || max.get().saturating_sub(pc.with(|pc| pc.health_dmg));

    view! {
        <div class= "relative">
            <div class= "stroke-red-600 fill-red-800" inner_html=icons::HEART />
            <h5 class= "absolute text-center top-0 w-full"> { curr } </h5>
        </div>
        <div class= "relative col-span-6">
            <input
                class= "range red-bar w-full h-full bg-inherit border-2 border-stone-400 pointer-events-none"
                type= "range"
                min=0
                max=max
                value=curr
            />
            <button
                class= "absolute cursor-pointer psuedo top-0 w-full h-full flex items-center justify-end"
                on:click=move |_| invert_raw()
            >
                <div class= "text-4xl px-4" hidden=is_not_raw inner_html= "&#x2022;" />
            </button>
        </div>
    }
}

fn heal_button() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let heal = move || {
        let (value, is_raw) = state.with(|state| (state.value, state.is_raw));
        logging::warn!("trying to heal {value}");
        pc.update(|pc| {
            if is_raw {
                pc.health_dmg = pc.health_dmg.saturating_sub(value);
            } else {
                pc.guard_dmg = pc.guard_dmg.saturating_sub(value);
            }
        });
        state.reset();
    };

    view! {
        <button
            class= "btn bg-green-800 rounded flex-center col-span-2"
            on:click=move |_| heal()
        >
            "HEAL"
        </button>
    }
}

fn user_input() -> impl IntoView {
    let state = State::expect();
    let value = move || state.with(|state| state.value);
    let display_value = move || {
        let value = value();
        (value > 0).then_some(value.to_string()).unwrap_or_default()
    };
    let set_value = move |ev: Event| {
        let parsed = event_target_value(&ev).parse::<u32>().unwrap_or_default();
        let parsed = cmp::min(parsed, 50);
        state.update(|state| state.value = parsed);
    };

    view! {
        <input
            class= "input col-span-3 text-center"
            type= "number"
            on:input=set_value
            prop:value=display_value
        />
    }
}

fn damage_button() -> impl IntoView {
    let (pc, sesh, state) = (PC::expect(), Session::expect(), State::expect());
    let apply_dmg = move || {
        let (damage, is_raw) = state.with(|state| (state.value, state.is_raw));
        let (max_guard, max_health) = sesh.with(|sesh| {
            let stam = sesh.abi_scores.get(Ability::Guard) as u32;
            let health = sesh.abi_scores.get(Ability::Health) as u32;
            (stam, health)
        });
        pc.update(|pc| {
            let mut apply_to_health = |dmg| {
                if pc.health_dmg + dmg > max_health {
                    pc.health_dmg = max_health;
                } else {
                    pc.health_dmg += dmg;
                }
            };

            if pc.guard_dmg >= max_guard || is_raw {
                apply_to_health(damage)
            } else if pc.guard_dmg + damage > max_guard {
                apply_to_health(damage - (max_guard - pc.guard_dmg));
                pc.guard_dmg = max_guard;
            } else {
                pc.guard_dmg += damage;
            }
        });
        state.reset()
    };

    view! {
        <button
            class= "btn bg-red-800 flex-center col-span-2"
            on:click=move |_| apply_dmg()
        >
            "DAMAGE"
        </button>
    }
}
