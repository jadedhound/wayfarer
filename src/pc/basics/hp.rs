use leptos::*;

use super::PC;
use crate::modal::MiddleModal;
use crate::pc::{PCSession, PCStat};
use crate::svg;
use crate::utils::{read_context, write_context};

#[component]
pub fn HP(cx: Scope) -> impl IntoView {
    let pc = read_context::<PC>(cx);
    let session = read_context::<PCSession>(cx);
    let modal_hidden = create_rw_signal(cx, true);
    let (hp_mod, hp_mod_set) = create_signal(cx, -1_i32);
    let max_hp = move || session.with(|s| *s.stats.get(PCStat::HP));

    view! {
        cx,
        <div class= "flex py-4">
            <button
                class= "rounded p-4 bg-emerald-800 flex-auto w-30"
                on:click=move |_| {
                    hp_mod_set.set(1);
                    modal_hidden.set(false);
                }
            >
                "Heal"
            </button>
            <div class= "flex flex-col items-center px-4">
                <div> "HP" </div>
                <h4>
                    { move || pc.with(|pc| { format!("{} / {}", pc.curr_hp, max_hp())}) }
                </h4>
            </div>
            <button
                class= "rounded p-4 bg-red-800 flex-auto w-30"
                on:click=move |_| {
                    hp_mod_set.set(-1);
                    modal_hidden.set(false);
                }
            >
                "Damage"
            </button>
        </div>
        <Modal hp_mod=hp_mod hidden=modal_hidden />
    }
}

#[component]
fn Modal(cx: Scope, hp_mod: ReadSignal<i32>, hidden: RwSignal<bool>) -> impl IntoView {
    let (change_by, change_by_set) = create_signal(cx, 1_i32);

    // Modify the PC hp values with the user given value
    let adj_hp = move |_| {
        write_context::<PC>(cx).update(|pc| {
            let change = hp_mod.get_untracked() * change_by.get_untracked();
            let max = read_context::<PCSession>(cx).with(|s| *s.stats.get(PCStat::HP));
            let applied = pc.curr_hp + change;
            if applied < 0 {
                pc.curr_hp = 0
            } else if applied > max {
                pc.curr_hp = max
            } else {
                pc.curr_hp = applied
            }
        });
        hidden.set(true);
    };

    // The button adjusts based on what action (heal/damage) was chosen
    let apply_btn = move || {
        hp_mod.with(|hp_mod| {
            let (svg, colour) = if *hp_mod < 1 {
                (svg::BLOOD, "bg-red-800")
            } else {
                (svg::HEALING, "bg-emerald-800")
            };
            view! {
                cx,
                <button
                    on:click=adj_hp
                    class= format!("{colour} flex-none p-2 rounded-full")
                >
                    <div class= "w-8" inner_html=svg />
                </button>
            }
            .into_view(cx)
        })
    };

    view! {
        cx,
        <MiddleModal hidden=hidden.read_only()>
            <div class= "p-4">
                <div class= "flex gap-2 bg-zinc-800">
                    <input
                        type= "number"
                        class= " w-12 grow outline-none bg-zinc-800 border-2 border-amber-600 px-2 rounded text-center"
                        prop:value=move || change_by.get()
                        on:input=move |ev| {
                            if let Ok(dmg) = event_target_value(&ev).parse::<i32>() {
                                change_by_set.set(dmg)
                            }
                        }
                        autofocus
                    />
                    { move || apply_btn() }
                </div>
            </div>
        </MiddleModal>
    }
}
