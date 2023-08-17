use std::matches;

use leptos::*;

use crate::items::food::Food;
use crate::items::item_spec::ItemSpec;
use crate::items::simple::FATIGUE;
use crate::items::Item;
use crate::pc::PC;
use crate::svg;
use crate::utils::{expect_rw, RwProvided};
use crate::views::modal::{ModalCentered, ModalState};

struct ChosenFood(Option<usize>);

pub(super) fn rest() -> impl IntoView {
    let pc = expect_rw::<PC>();
    let chosen = create_rw_signal(ChosenFood(None));
    provide_context(chosen);
    let has_no_food = move || {
        pc.with(|pc| {
            !pc.inventory
                .values()
                .any(|item| matches!(item.spec, ItemSpec::Food(_)))
        })
    };
    let food_view = move || {
        if let Some(id) = chosen.with(|x| x.0) {
            pc.with(|pc| pc.inventory.get(id).unwrap().into_view())
        } else {
            let curr_status = if has_no_food() {
                "NO FOOD IN INVENTORY"
            } else {
                "NO FOOD CHOSEN"
            };
            view! {
                <div class= "text-left">
                    <div class= "title"> { curr_status } </div>
                    <div class= "italic text-sm"> "Fatigue won't be removed." </div>
                </div>
            }
            .into_view()
        }
    };

    view! {
        <div class= "flex gap-1">
            <button
                class= "bg-zinc-800 rounded w-12 grow p-2 border-2 border-zinc-800 disabled:bg-inherit"
                on:click=move |_| ModalState::open( 0)
                disabled=has_no_food
            >
                { food_view }
            </button>
            <button
                class= "border-2 border-emerald-600 rounded w-12 flex-centered"
                on:click=move |_| {
                    let id = chosen.with(|c| c.0);
                    chosen.update(|c| { c.0 = None });
                    pc.update(|pc| commit_rest( pc, id))
                }
            >
                <div class= "fill-emerald-600 w-6" inner_html=svg::CAMPFIRE />
            </button>
        </div>
        { modal }
    }
}

fn modal() -> impl IntoView {
    let chosen = expect_rw::<ChosenFood>();
    let food_filter = move || {
        PC::with(|pc| {
            pc.inventory
                .iter()
                .filter(|(_, item)| matches!(item.spec, ItemSpec::Food(_)))
                .map(|(id, item)| {
                    view! {
                        <button
                            class= "w-full py-2"
                            on:click=move |_| {
                                chosen.update(|x| x.0 = Some(id));
                                ModalState::dismiss()
                            }
                        >
                            { item }
                        </button>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <ModalCentered title=|| "FOOD" id=0>
            <div class= "shaded-table-light">
                { food_filter }
            </div>
        </ModalCentered>
    }
}

/// Progresses time, removes expired buffs and applies the food
/// given by the `id`.
fn commit_rest(pc: &mut PC, id: Option<usize>) {
    // Progress turns by a day.
    pc.turns.next_day();
    // Add buffs, remove fatigue and adjust food stock.
    if let Some(id) = id {
        let Food { buff, fatigue } = pc
            .inventory
            .get(id)
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
            .collect();
        for id in filtered {
            pc.inventory.remove(id);
        }
        // Add buff.
        if let Some(mut buff) = buff {
            buff.set_duration();
            pc.buffs.add(buff)
        }
        // Remove food or adjust stacks.
        let food_item = pc.inventory.get_mut(id).unwrap();
        if let Some(x) = food_item.stacks.as_mut() {
            if x.0 > 1 {
                x.0 -= 1;
            } else {
                pc.inventory.remove(id);
            }
        } else {
            pc.inventory.remove(id);
        };
    }
}
