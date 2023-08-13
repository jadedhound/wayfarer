use std::matches;

use leptos::*;

use crate::items::food::Food;
use crate::items::item_spec::ItemSpec;
use crate::items::simple::FATIGUE;
use crate::items::Item;
use crate::pc::PC;
use crate::utils::rw_context;
use crate::views::modal::{ModalCentered, ModalState};
use crate::{css, svg};

struct ChosenFood(Option<usize>);

#[component]
pub(super) fn Rest(cx: Scope) -> impl IntoView {
    let pc = rw_context::<PC>(cx);
    let chosen = create_rw_signal(cx, ChosenFood(None));
    let food_view = move || {
        if let Some(id) = chosen.with(|x| x.0) {
            pc.with(|pc| pc.inventory.get(&id).unwrap().into_view(cx))
        } else {
            view! { cx,
                <div class= "text-left">
                    <div class= "font-sans"> "NO FOOD CHOSEN" </div>
                    <div> "Fatigue won't be removed." </div>
                </div>
            }
            .into_view(cx)
        }
    };
    let has_no_food = move || {
        pc.with(|pc| {
            !pc.inventory
                .values()
                .any(|item| matches!(item.spec, ItemSpec::Food(_)))
        })
    };
    let food_filter = move || {
        pc.with(|pc| {
            pc.inventory
                .iter()
                .filter(|(_, item)| matches!(item.spec, ItemSpec::Food(_)))
                .map(|(id, item)| {
                    let id = *id;
                    view! { cx,
                        <button
                            class=css::BTN_LIGHT
                            on:click=move |_| {
                                chosen.update(|x| x.0 = Some(id));
                                ModalState::dismiss(cx)
                            }
                        >
                            { item }
                        </button>
                    }
                })
                .collect_view(cx)
        })
    };

    view! { cx,
        <div class= "flex">
            <button
                class= "bg-zinc-800 rounded-l w-full px-2 py-2 disabled:bg-inherit disabled:border-l-2 disabled:border-y-2 disabled:border-zinc-800"
                on:click=move |_| ModalState::open(cx, 0)
                disabled=has_no_food
            >
                { food_view }
            </button>
            <button
                class= "bg-red-800 rounded-r w-12 flex-centered"
                on:click=move |_| {
                    let id = chosen.with(|c| c.0);
                    chosen.update(|c| { c.0 = None });
                    pc.update(|pc| rest(cx, pc, id))
                }
            >
                <div class= "svg w-6" inner_html=svg::CAMPFIRE />
            </button>
        </div>

        <ModalCentered title=|| "Food" id=0>
            { food_filter }
        </ModalCentered>
    }
}

/// Progresses time, removes expired buffs and applies the food
/// given by the `id`.
fn rest(cx: Scope, pc: &mut PC, id: Option<usize>) {
    // Progress turns by a day.
    pc.turns += 24 * 60 * 6;
    // Remove expired buffs.
    let expired: Vec<_> = pc
        .buffs
        .iter()
        .filter(|(_, buff)| buff.duration < pc.turns)
        .map(|(id, _)| id)
        .copied()
        .collect();
    for id in expired {
        pc.buffs.remove(&id);
    }
    // Add buffs, remove fatigue and adjust food stock.
    if let Some(id) = id {
        let Food { buff, fatigue } = pc
            .inventory
            .get(&id)
            .and_then(|x| Some(x.spec.as_food()?.clone()))
            .unwrap();
        // Remove required fatigue.
        let fatigue_ref: Item = FATIGUE.into();
        let filtered: Vec<_> = pc
            .inventory
            .iter()
            .filter(|(_, item)| *item == &fatigue_ref)
            .map(|(id, _)| id)
            .take(fatigue as usize)
            .copied()
            .collect();
        for id in filtered {
            pc.inventory.remove(&id);
        }
        // Add buff.
        if let Some(buff) = buff {
            pc.buffs.push(buff.set_duration(cx))
        }
        // Remove food or adjust stacks.
        let food_item = pc.inventory.get_mut(&id).unwrap();
        if let Some(x) = food_item.stacks.as_mut() {
            if x.0 > 1 {
                x.0 -= 1;
            } else {
                pc.inventory.remove(&id);
            }
        } else {
            pc.inventory.remove(&id);
        };
    }
}
