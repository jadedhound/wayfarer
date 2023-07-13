use leptos::*;

use crate::items::{Armour, Item};
use crate::pc::inventory::inv_item::InvItem;
use crate::pc::{EquipSlot, PC};
use crate::svg;
use crate::utils::{read_context, rw_context, write_context, Modal, StrPlus};

/// The state of the choose equipment modal: (is hidden, which slot)
struct ModalState(bool, EquipSlot);

#[component]
pub fn Equipment(cx: Scope) -> impl IntoView {
    let hidden = create_rw_signal(cx, ModalState(true, EquipSlot::MainHand));
    provide_context(cx, hidden);
    view! {
        cx,
        <h4 class= "border-b-2 border-purple-900 mb-4 text-center"> "Equipment" </h4>
        <div class= "grid grid-cols-7 gap-2">
            <EqSlot svg= svg::SWORD eq_slot= EquipSlot::MainHand />
            <EqSlot svg= svg::SHIELD eq_slot= EquipSlot::OffHand />
            <EqSlot svg= svg::HELM eq_slot= EquipSlot::Head />
            <EqSlot svg= svg::ARMOUR eq_slot= EquipSlot::Body />
            <EqSlot svg= svg::BOOTS eq_slot= EquipSlot::Legs />
        </div>
        <ChangeEqModal />
    }
}

#[component]
fn EqSlot(cx: Scope, svg: &'static str, eq_slot: EquipSlot) -> impl IntoView {
    let pc = read_context::<PC>(cx);
    let item = move || {
        pc.with(|pc| {
            pc.get_equip(eq_slot)
                .map(|item| {
                    view! {
                        cx,
                        <InvItem item />
                    }
                    .into_view(cx)
                })
                .unwrap_or_default()
        })
    };

    let modal_state = rw_context::<ModalState>(cx);
    view! {
        cx,
        <div class= "rounded border-2 border-purple-900 h-12 flex-centered">
            <div class= "w-6" inner_html=svg/>
        </div>
        <button
            class= "col-span-6 rounded bg-zinc-800 gap-2"
            on:click= move |_| modal_state.update(|c| {
                c.0 = false;
                c.1 = eq_slot;
            })
        >
            { move || item() }
        </button>
    }
}

/// Filters out equiped items and gives their index in the original list.
fn not_equiped_with_index(pc: &PC) -> Vec<(usize, Item)> {
    let mut result = Vec::new();
    let equiped: Vec<&usize> = pc.equipment.0.iter().flatten().collect();
    for (i, item) in pc.inventory.iter().enumerate() {
        if !equiped.contains(&&i) {
            result.push((i, item.clone()))
        }
    }
    result
}

/// Fliters the inventory by the `slot` given and give the associated
/// original inventory index.
fn filter_by_slot(pc: &PC, slot: EquipSlot) -> Vec<(usize, Item)> {
    not_equiped_with_index(pc)
        .into_iter()
        .filter(|(_, x)| match slot {
            EquipSlot::Head => matches!(x, Item::Head(_)),
            EquipSlot::MainHand => matches!(x, Item::Held(_)),
            EquipSlot::OffHand => matches!(x, Item::Held(_)),
            EquipSlot::Body => match x {
                Item::Armour(y) => matches!(
                    y.base,
                    Armour::Robe | Armour::Gambeson | Armour::Brigandine | Armour::Plate
                ),
                _ => false,
            },
            EquipSlot::Legs => match x {
                Item::Armour(y) => matches!(
                    y.base,
                    Armour::Leggings | Armour::Chausses | Armour::Greaves
                ),
                _ => false,
            },
        })
        .collect()
}

#[component]
fn ChangeEqModal(cx: Scope) -> impl IntoView {
    const BTN_CSS: &str = "rounded bg-zinc-700 h-12";
    let chg_modal = rw_context::<ModalState>(cx);
    let hidden = move || chg_modal.with(|c| c.0);
    let close_modal = move || chg_modal.update(|c| c.0 = true);
    let set_slot = move |i| {
        chg_modal.with_untracked(|ModalState(_, slot)| {
            write_context::<PC>(cx).update(|pc| *pc.equipment.get_mut(*slot) = i)
        });
        close_modal();
    };
    let items = move || {
        chg_modal.with(|ModalState(_, slot)| {
            read_context::<PC>(cx).with_untracked(|pc| filter_by_slot(pc, *slot))
        })
    };
    let item_view = move || {
        items()
            .into_iter()
            .map(|(i, item)| {
                view! {
                    cx,
                    <button
                        class=BTN_CSS
                        on:click=move |_| set_slot(Some(i))
                    >
                        <InvItem item />
                    </button>
                }
            })
            .collect_view(cx)
    };

    view! {
        cx,
        <Modal hidden=move || hidden()>
            <div class= "flex-centered h-cover px-4">
                <div class= "relative flex flex-col bg-zinc-800 w-full p-4 rounded">
                    <button
                        class= "w-10 absolute top-0 right-0"
                        on:click= move |_| close_modal()
                        inner_html=svg::CROSS
                    />
                    <h4 class= "text-center">
                        { move || chg_modal.with(|state| state.1.to_string())}
                    </h4>
                    <div class= "grid grid-cols-1 auto-rows-fr gap-2">
                        { move || item_view() }
                        <button class=BTN_CSS.plus("px-2 text-start") on:click=move |_| set_slot(None) >
                            "None"
                        </button>
                    </div>
                </div>
            </div>
        </Modal>
    }
}
