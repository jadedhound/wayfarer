use const_format::concatcp;
use leptos::*;

use super::PC;
use crate::pc::session::PCSession;
use crate::pc::PCStat;
use crate::svg;
use crate::utils::{expect_rw, RwProvided};

const DISABLED_BTN: &str = "disabled:bg-inherit";

struct RawDamage(bool);

pub fn hp() -> impl IntoView {
    provide_context(create_rw_signal(RawDamage(false)));

    view! {
        <div class= "flex items-stretch gap-1 py-2">
            { heal_btns() }
            { health_and_stam() }
            { damage_input() }
        </div>
    }
}

fn heal_btns() -> impl IntoView {
    let stam_disabled = move || PC::with(|pc| pc.stamina_dmg == 0);
    let health_disabled = move || PC::with(|pc| pc.health_dmg == 0);
    let svg_class = move |disabled| {
        if disabled {
            "w-8 fill-zinc-500"
        } else {
            "w-8 svg"
        }
    };

    view! {
        <div class= "flex flex-col gap-1">
            <button
                class= "rounded p-4 bg-emerald-800 flex-centered disabled:bg-inherit"
                on:click=move |_| PC::update( |pc| pc.stamina_dmg = 0)
                disabled=stam_disabled
            >
                <div class=move || svg_class(stam_disabled()) inner_html=svg::CAMPFIRE />
            </button>
            <button
                class=concatcp!("rounded p-4 bg-emerald-800 flex-centered ", DISABLED_BTN)
                on:click=move |_| {
                    PC::update( |pc| {
                       if pc.health_dmg > 0 {
                            pc.health_dmg -= 1
                        }
                    })
                }
                disabled=health_disabled
            >
                <div class=move || svg_class(health_disabled()) inner_html=svg::HEALING />
            </button>
        </div>
    }
}

fn curr_stam() -> i32 {
    let stam_dmg = PC::with(|pc| pc.stamina_dmg);
    let max = PCSession::with(|sesh| sesh.stats.get(PCStat::Stamina));
    max - stam_dmg
}

fn curr_health() -> i32 {
    let health_dmg = PC::with(|pc| pc.health_dmg);
    let max = PCSession::with(|sesh| sesh.stats.get(PCStat::Health));
    max - health_dmg
}

fn health_and_stam() -> impl IntoView {
    let raw_dmg = expect_rw::<RawDamage>();
    let invert_raw_dmg = move || raw_dmg.update(|x| x.0 = !x.0);
    let is_raw = move || raw_dmg.with(|x| x.0);

    view! {
        <div class= "flex flex-col gap-1 grow w-12">
            <div class= "flex-centered flex-col h-16">
                <div class= "text-sm"> "Stamina" </div>
                <h4> { curr_stam } </h4>
            </div>
            <button
                class= "flex-centered flex-col h-16 relative"
                on:click=move |_| invert_raw_dmg()
            >
                <div class= "absolute left-0 inset-y-0" hidden=move || !is_raw()>
                    <div class= "flex-centered h-full ml-4">
                        <div class= "fill-red-800 w-6" inner_html=svg::BLOOD />
                    </div>
                </div>
                <div class= "text-sm"> "Health" </div>
                <h4> { curr_health } </h4>
            </button>
        </div>
    }
}

fn damage_input() -> impl IntoView {
    let hp_dmg = create_rw_signal(1_i32);
    let raw_dmg = expect_rw::<RawDamage>();
    let apply_dmg = move || {
        let dmg = hp_dmg.get();
        let (max_stam, max_health) = PCSession::with(|sesh| {
            let stam = sesh.stats.get(PCStat::Stamina);
            let health = sesh.stats.get(PCStat::Health);
            (stam, health)
        });
        let is_raw_dmg = raw_dmg.with(|x| x.0);
        PC::update(|pc| {
            let mut apply_to_health = |dmg| {
                if pc.health_dmg + dmg > max_health {
                    pc.health_dmg = max_health;
                } else {
                    pc.health_dmg += dmg;
                }
            };
            if pc.stamina_dmg >= max_stam || is_raw_dmg {
                apply_to_health(dmg)
            } else if dmg > max_stam {
                pc.stamina_dmg = max_stam;
                apply_to_health(dmg - max_stam)
            } else {
                pc.stamina_dmg += dmg;
            }
        });
        hp_dmg.set(1);
        raw_dmg.update(|x| x.0 = false);
    };

    view! {
        <button
            class= "rounded bg-red-800 w-14 row-span-2 flex-centered font-sans"
            on:click=move |_| apply_dmg()
        >
            { move || format!("-{}", hp_dmg.get()) }
        </button>
        <div class= "relative w-14 flex flex-col justify-between items-center">
            <input
                class= "hp-dmg-range"
                type= "range"
                min=1
                max=12
                on:input=move |ev| { hp_dmg.set(event_target_value(&ev).parse::<i32>().unwrap_or(1)) }
                prop:value=move || hp_dmg.get()
            />
            <div class= "svg w-4 mt-2 z-[1] cursor-none" inner_html=svg::PLUS />
            <div class= "svg w-4 mb-2 z-[1] cursor-none" inner_html=svg::MINUS />
        </div>
    }
}
