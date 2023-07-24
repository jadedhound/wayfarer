use std::vec;

use leptos::*;
use strum::IntoEnumIterator;

use super::Artisan;
use crate::items::craft::{craft, Experience};
use crate::items::Item;
use crate::pc::crafting::supply::Supply;
use crate::pc::crafting::CraftState;
use crate::pc::PC;
use crate::utils::{read_context, rw_context};
use crate::views::modal::*;
use crate::views::{InvItem, InvItemByIndex};

enum ModalState {
    Artisan,
    Experience,
    Item,
    None,
}

impl ModalState {
    fn close(cx: Scope) {
        rw_context::<ModalState>(cx).update(|a| *a = ModalState::None)
    }
}

#[component]
pub fn Components(cx: Scope) -> impl IntoView {
    let state = rw_context::<CraftState>(cx);
    let modal = create_rw_signal(cx, ModalState::None);
    provide_context(cx, modal);

    view! {
        cx,
        <div class= "grid grid-cols-5 gap-2 text-center items-center">
            <div class= "text-xl flex-centered col-span-2"> "Artisan" </div>
            <button
                class= "bg-amber-800 col-span-3 rounded h-12 text-left px-2"
                on:click=move |_| modal.update(|a| *a = ModalState::Artisan)
            >
                { move || state.with(|s| s.artisan.to_string() )}
            </button>
            <div class= "text-xl flex-centered col-span-2"> "Experience" </div>
            <button
                class= "bg-amber-800 col-span-3 rounded h-12 text-left px-2"
                on:click=move |_| modal.update(|a| *a = ModalState::Experience)
            >
                { move || state.with(|s| s.exp.to_string() )}
            </button>
            <div class= "text-xl flex-centered col-span-2"> "Item" </div>
            <button
                class= "bg-amber-800 col-span-3 rounded h-12 disabled:bg-zinc-900 py-1"
                on:click=move |_| modal.update(|a| *a = ModalState::Item)
            >
                { move || state.with(|s| match s.item.as_ref() {
                    Some(i) => view!{ cx, <InvItemByIndex i=*i /> }.into_view(cx),
                    None => view!{ cx, }.into_view(cx)
                })}
            </button>
            <Supply />
        </div>

        // POPUP HIDDEN MENUS
        <ModalMenu
            hidden=move || modal.with(|a| !matches!(a, ModalState::Artisan))
            dismiss=move || ModalState::close(cx)
            title= "Artisan"
        >
                { move || artisan_arr(cx) }
        </ModalMenu>
        <ModalMenu
            hidden=move || modal.with(|a| !matches!(a, ModalState::Experience))
            dismiss=move || ModalState::close(cx)
            title= "Artisan Experience"
        >
                <ExpMenu />
        </ModalMenu>
        <ModalMenu
            hidden=move || modal.with(|a| !matches!(a, ModalState::Item))
            dismiss=move || ModalState::close(cx)
            title= "Items"
        >
            { move || items_arr(cx) }
        </ModalMenu>
    }
}

#[component]
fn ExpMenu(cx: Scope) -> impl IntoView {
    let state = rw_context::<CraftState>(cx);
    let exp = Experience::iter()
        .map(|e| {
            view! { cx,
                <button
                    class=MODAL_MENU_BTN
                    on:click=move |_| {
                        state.update(|s| s.exp = e);
                        ModalState::close(cx)
                    }
                >
                    { e.to_string() }
                </button>
            }
        })
        .collect_view(cx);
    view! { cx,
        { exp }
    }
}

fn artisan_arr(cx: Scope) -> View {
    vec![Artisan::Enchanter, Artisan::Chef, Artisan::Alchemist]
        .into_iter()
        .map(|a| {
            let b = a.to_string();
            view! {
                cx,
                <button
                    class=MODAL_MENU_BTN
                    on:click=move |_| {
                        rw_context::<CraftState>(cx).update(|s| s.artisan = a);
                        ModalState::close(cx);
                    }
                >
                    { b }
                </button>
            }
        })
        .collect_view(cx)
}

fn items_arr(cx: Scope) -> View {
    let state = rw_context::<CraftState>(cx);
    let artisan = state.with(|s| s.artisan);
    let pc = read_context::<PC>(cx);
    // This modal should only be used when an artisan has been chosen
    let by_artisan = |(_, item): &(usize, &Item)| match artisan {
        Artisan::Enchanter => matches!(item, Item::Held(_) | Item::Armour(_)),
        Artisan::Alchemist => todo!(),
        Artisan::Chef => todo!(),
    };
    let items: Vec<(usize, Item)> = pc.with(|pc| {
        pc.inventory
            .iter()
            .enumerate()
            .filter(by_artisan)
            .map(|(i, item)| (i, item.clone()))
            .collect()
    });
    items
        .into_iter()
        .map(|(i, item)| {
            let item = item.clone();
            view! {
                cx,
             <button
                class=MODAL_MENU_BTN
                on:click=move |_| {
                    state.update(|s| {
                        s.chg_item(cx, i);
                    });
                    ModalState::close(cx);
                }
            >
                <InvItem item=item />
            </button>       }
        })
        .collect_view(cx)
}
