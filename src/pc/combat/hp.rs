use std::cmp;

use leptos::*;

use super::PC;
use crate::icons;
use crate::pc::session::Session;
use crate::pc::{update, PCStat};
use crate::utils::expect_rw;
use crate::utils::rw_utils::RwUtils;

#[derive(Clone, Copy)]
struct State {
    is_raw: bool,
    been_interacted: bool,
    abs_value: i32,
}

/// Each time the user clicks a button (heal or damage) to confirm a choice
/// the state is reset to this default.
impl Default for State {
    fn default() -> Self {
        Self {
            is_raw: false,
            been_interacted: false,
            abs_value: 1,
        }
    }
}

impl RwUtils for State {
    type Item = Self;
}

pub fn hp() -> impl IntoView {
    State::provide();

    view! {
        { guard }
        { health }
        { heal_btn }
        { input_range }
        { damage_btn }
    }
}

fn guard() -> impl IntoView {
    let pc = PC::expect();
    let max = Session::slice(|sesh| sesh.stats.get(PCStat::Guard));
    let curr = move || {
        let stam_dmg = pc.with(|pc| pc.guard_dmg);
        max.get() - stam_dmg
    };
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
    let max = Session::slice(|sesh| sesh.stats.get(PCStat::Health));
    let curr = move || {
        let health_dmg = pc.with(|pc| pc.health_dmg);
        max.get() - health_dmg
    };

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

fn heal_btn() -> impl IntoView {
    let (pc, sesh, state) = (PC::expect(), Session::expect(), State::expect());
    let is_max_hp = PC::slice(|pc| pc.health_dmg == 0);
    let is_max_guard = create_memo(move |_| {
        let no_damage = pc.with(|pc| pc.guard_dmg < 1);
        let no_guard = sesh.with(|sesh| sesh.stats.get(PCStat::Guard) < 1);
        no_damage || no_guard
    });
    let been_interacted = create_read_slice(state, |x| x.been_interacted);
    let abs_value = create_read_slice(state, |x| x.abs_value);
    let is_disabled = move || is_max_guard.get() && is_max_hp.get();

    let icon = move || {
        let icon = if is_max_guard.get() {
            icons::HEALING
        } else {
            icons::FIST
        };
        view! {
            <div class= "w-6" inner_html=icon />
        }
    };
    let icon_or_text = move || {
        if been_interacted.get() {
            format!("+{}", abs_value.get()).into_view()
        } else {
            icon.into_view()
        }
    };
    let heal = move || {
        let val = abs_value.get();
        let mut rally_update = false;
        pc.update(|pc| {
            if been_interacted.get() {
                pc.guard_dmg = cmp::max(pc.guard_dmg - val, 0);
            } else if is_max_guard.get() {
                pc.health_dmg -= 1
            } else {
                rally_update = true;
                pc.guard_dmg = 0
            }
        });
        if rally_update {
            update::on_rally();
        }
        state.set(State::default());
    };

    view! {
        <button
            class= "btn bg-green-800 rounded flex-center"
            on:click=move |_| heal()
            disabled=is_disabled
        >
            { icon_or_text }
        </button>
    }
}

fn input_range() -> impl IntoView {
    let state = expect_rw::<State>();
    let update_val = move |val: String| {
        let val = val.parse::<i32>().unwrap_or(1);
        state.update(|state| {
            state.been_interacted = true;
            state.abs_value = val
        })
    };
    let val = create_read_slice(state, |state| state.abs_value);

    view! {
        <div class= "relative h-12 col-span-5 flex items-center justify-between px-2 pointer-events-none">
            <div class= "z-[1] w-4" inner_html=icons::MINUS />
            <input
                class= "absolute left-0 range yellow-thumb bg-yellow-900 yellow-bar w-full h-full pointer-events-auto"
                type= "range"
                min=1
                max=12
                on:input=move |ev| update_val(event_target_value(&ev))
                prop:value=val
            />
            <div class= "z-[1] w-4" inner_html=icons::PLUS />
        </div>
    }
}

fn damage_btn() -> impl IntoView {
    let (pc, sesh, state) = (PC::expect(), Session::expect(), State::expect());
    let val_slice = create_read_slice(state, |state| state.abs_value);
    let apply_dmg = move || {
        let (dmg, is_raw) = state.with(|x| (x.abs_value, x.is_raw));
        let (max_stam, max_health) = sesh.with(|sesh| {
            let stam = sesh.stats.get(PCStat::Guard);
            let health = sesh.stats.get(PCStat::Health);
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

            if pc.guard_dmg >= max_stam || is_raw {
                apply_to_health(dmg)
            } else if dmg + pc.guard_dmg > max_stam {
                apply_to_health(dmg - (max_stam - pc.guard_dmg));
                pc.guard_dmg = max_stam;
            } else {
                pc.guard_dmg += dmg;
            }
        });
        state.set(State::default())
    };

    view! {
        <button
            class= "btn bg-red-800 flex-center"
            on:click=move |_| apply_dmg()
        >
            { move || format!("-{}", val_slice.get()) }
        </button>
    }
}
