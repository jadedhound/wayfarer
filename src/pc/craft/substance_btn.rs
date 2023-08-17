use super::CraftState;
use crate::items::Item;
use crate::svg;
use crate::utils::RwProvided;
use crate::views::modal::{ModalCentered, ModalState};
use crate::{pc::PC, utils::expect_rw};
use leptos::*;

pub(super) fn substance_view(is_first: bool) -> impl IntoView {
    let state = expect_rw::<CraftState>();
    let sub = state.with(|x| x.recipe.substances[is_first as usize]);
    let chosen = create_memo(move |_| state.with(|x| x.chosen[is_first as usize]));

    let item_view = move || {
        if let Some(id) = chosen.get() {
            PC::with(|pc| pc.inventory.get(id).unwrap().into_view())
        } else {
            view! {
                <div class= "font-sans text-zinc-500 uppercase">
                    { sub.to_string() }
                </div>
            }
            .into_view()
        }
    };
    let no_items = create_memo(move |_| {
        PC::with(|pc| {
            // If the item has already been chosen, then ensure it
            // has enough stacks to be chosen again.
            let cannot_choose = state.with(|x| {
                x.chosen[!is_first as usize].filter(|&id| {
                    PC::with(|pc| {
                        pc.inventory
                            .get(id)
                            .unwrap()
                            .stacks
                            .is_some_and(|(curr, _)| curr < 2)
                    })
                })
            });
            !pc.inventory.iter().any(|(id, item)| {
                if cannot_choose.is_some_and(|x| x == id) {
                    false
                } else {
                    item.spec.as_reagent().is_some_and(|x| x.get(sub).is_some())
                }
            })
        })
    });
    let edit_icon = move || {
        if chosen.get().is_some() {
            view! {
                <div class= "stroke-red-600 w-4" inner_html=svg::CROSS />
            }
        } else if no_items.get() {
            view! {
                <div class= "stroke-red-600 w-4" inner_html=svg::CANCEL />
            }
        } else {
            view! {
                <div class= "svg w-4" inner_html=svg::QUILL />
            }
        }
    };
    let click_action = move || {
        if chosen.get().is_some() {
            state.update(|x| {
                x.chosen[is_first as usize].take();
            });
        } else {
            state.update(|x| x.change_first = is_first);
            ModalState::open(1)
        }
    };

    view! {
        <button
            class= "flex items-center gap-2 px-2 py-4 text-left"
            on:click=move |_| click_action()
            disabled=move || no_items.get()
        >
            <div class= "svg w-6" inner_html=sub.to_svg() />
            <div class= "ml-2 w-12 grow"> { item_view } </div>
            { edit_icon }
        </button>
    }
}

pub(super) fn modal_substance_filter() -> impl IntoView {
    let state = expect_rw::<CraftState>();
    let sub = move || state.with(|x| x.recipe.substances[x.change_first as usize]);
    let filtered = move || {
        let prev_chosen: Vec<usize> = state.with(|x| {
            let curr = x.chosen[x.change_first as usize];
            let other = x.chosen[!x.change_first as usize].and_then(|id| {
                PC::with(|pc| {
                    let can_choose = pc
                        .inventory
                        .get(id)
                        .unwrap()
                        .stacks
                        .is_some_and(|(curr, _)| curr > 1);
                    if can_choose {
                        None
                    } else {
                        Some(id)
                    }
                })
            });
            [curr, other].into_iter().flatten().collect()
        });
        PC::with(|pc| {
            let mut x: Vec<_> = pc
                .inventory
                .iter()
                .filter_map(|(id, item)| {
                    if prev_chosen.contains(&id) {
                        None
                    } else {
                        let sub_am = item.spec.as_reagent()?.get(sub())?;
                        Some((id, item, sub_am))
                    }
                })
                .collect();
            x.sort_unstable_by(|a, b| a.2.cmp(&b.2));
            x.iter()
                .rev()
                .map(|(id, item, _)| item_btn(*id, item))
                .collect_view()
        })
    };
    view! {
        <ModalCentered title=sub id=1>
            <div class= "shaded-table-light">
                { filtered }
            </div>
        </ModalCentered>
    }
}

fn item_btn(id: usize, item: &Item) -> impl IntoView {
    let state = expect_rw::<CraftState>();
    let update_state = move || {
        state.update(|x| x.chosen[x.change_first as usize] = Some(id));
        ModalState::dismiss()
    };

    view! {
        <button
            class= "w-full"
            on:click=move |_| update_state()
        >
            { item }
        </button>
    }
}
