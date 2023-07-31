use const_format::concatcp;
use leptos::*;

use super::PC;
use crate::items::simple::FATIGUE;
use crate::pc::session::PCSession;
use crate::pc::PCStat;
use crate::svg;
use crate::utils::rw_context;
use crate::views::toast::{Toast, ToastNotif};

fn apply_damage(pc: &mut PC, dmg: i32) {
    if dmg > pc.curr_hp {
        pc.wounds += dmg - pc.curr_hp;
        pc.curr_hp = 0;
    } else {
        pc.curr_hp -= dmg
    }
}

#[allow(unused_braces)]
fn hp_number_or_dc(cx: Scope, pc: &PC, max: i32) -> View {
    let curr = pc.curr_hp;
    let wounds = pc.wounds;
    if curr > 0 {
        view! {
            cx,
            { format!("{curr} / {max}") }
        }
        .into_view(cx)
    } else if wounds < 15 {
        view! {
            cx,
            <span class= "text-2xl">
                { format!("STR DC {}", 10 + wounds) }
            </span>
        }
        .into_view(cx)
    } else {
        view! {
            cx,
            "Dead"
        }
        .into_view(cx)
    }
}

#[component]
pub fn HP(cx: Scope) -> impl IntoView {
    const DISABLED_BTN: &str =
        "border-2 border-emerald-800 disabled:bg-inherit disabled:border-zinc-700";
    let pc = rw_context::<PC>(cx);
    let max_hp = move || rw_context::<PCSession>(cx).with(|s| s.stats[PCStat::HP.index()]);
    let (hp_dmg, hp_dmg_set) = create_signal(cx, 1_i32);

    view! {
        cx,
        <ToastNotif />
        <div class= "flex gap-1 py-4">
            <div class= "flex flex-col gap-1">
                <button
                    class=concatcp!("rounded p-4 bg-emerald-800 flex-centered ", DISABLED_BTN)
                    on:click=move |_| {
                        // Reset HP and add fatigue
                        pc.update(|pc| {
                            pc.curr_hp = max_hp();
                            pc.inventory.push(FATIGUE.into());
                        });
                        Toast::show(cx, "Fatigue added to inventory");
                    }
                    disabled=move || pc.with(|pc| pc.curr_hp == max_hp())
                >
                    <div class= "w-8 svg" inner_html=svg::CAMPFIRE />
                </button>
                <button
                    class=concatcp!("rounded p-4 bg-emerald-800 flex-centered ", DISABLED_BTN)
                    on:click=move |_| {
                        pc.update(|pc| {
                           if pc.wounds > 0 {
                                pc.wounds -= 1
                            }
                        })
                    }
                    disabled=move || pc.with(|pc| pc.wounds == 0)
                >
                    <div class= "w-8 svg" inner_html=svg::HEALING />
                </button>
            </div>
            <div class= "flex flex-col gap-1 grow w-12">
                <div class= "text-center px-4 col-span-2">
                    <div class= "text-sm"> "HP" </div>
                    <h4>
                        { move || pc.with(|pc| hp_number_or_dc(cx, pc, max_hp())) }
                    </h4>
                </div>
                <div class= "text-center px-4 col-span-2">
                    <div class= "text-sm"> "Wounds" </div>
                    <h4>
                        { move || pc.with(|pc| {
                            if pc.wounds > 15 {
                                "Dead".to_string()
                            } else {
                                pc.wounds.to_string()
                            }
                        }) }
                    </h4>
                </div>
            </div>
            <button
                class= "rounded bg-red-800 w-14 row-span-2 flex-centered"
                on:click=move |_| {
                    pc.update(|pc| apply_damage(pc, hp_dmg.get_untracked()));
                    hp_dmg_set.set(1);
                }
            >
                <div>
                    { move || format!("-{}", hp_dmg.get()) }
                </div>
            </button>
            <div class= "psuedo relative w-12 h-32">
                <input
                    class= "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 vert-slider w-32 h-12"
                    type= "range"
                    min=1
                    max=12
                    on:input=move |ev| { hp_dmg_set.set(event_target_value(&ev).parse::<i32>().unwrap_or(1)) }
                    prop:value=move || hp_dmg.get()
                />
            </div>
        </div>
    }
}
