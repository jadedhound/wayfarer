use const_format::concatcp;
use leptos::*;

use crate::items::armour::BodyPart;
use crate::items::item_specs::ItemSpec;
use crate::items::Item;
use crate::pc::{session, EquipSlot, PC};
use crate::svg;
use crate::utils::rw_context;
use crate::views::modal::{ModalCentered, ModalState};

struct ChgSlot(EquipSlot);

#[component]
pub fn Equipment(cx: Scope) -> impl IntoView {
    let chg = create_rw_signal(cx, ChgSlot(EquipSlot::MainHand));
    provide_context(cx, chg);
    let cant_off_hand = move || {
        rw_context::<PC>(cx).with(|pc| {
            pc.get_equipment(EquipSlot::MainHand)
                .is_some_and(|x| x.weight > 1)
        })
    };
    view! {
        cx,
        <div class= "grid grid-cols-7 gap-2">
            <EqSlot svg= svg::SWORD eq_slot= EquipSlot::MainHand />
            { move || if cant_off_hand() {
                ().into_view(cx)
            } else {
                view!{ cx, <EqSlot svg= svg::SHIELD eq_slot= EquipSlot::OffHand /> }.into_view(cx)
            }}
            <EqSlot svg= svg::HELM eq_slot= EquipSlot::Head />
            <EqSlot svg= svg::ARMOUR eq_slot= EquipSlot::Body />
            <EqSlot svg= svg::BOOTS eq_slot= EquipSlot::Legs />
            <EqSlot svg= svg::MORTAR_PESTLE eq_slot= EquipSlot::Tools />
        </div>
        <ChangeEqModal />
    }
}

#[component]
fn EqSlot(cx: Scope, svg: &'static str, eq_slot: EquipSlot) -> impl IntoView {
    let pc = rw_context::<PC>(cx);
    let item = move || {
        pc.with(|pc| {
            pc.equipment[eq_slot as usize]
                .as_ref()
                .map(|item| item.into_view(cx))
                .unwrap_or_default()
        })
    };

    view! {
        cx,
        <div class= "rounded border-2 border-purple-900 py-2 flex-centered">
            <div class= "w-6 svg" inner_html=svg/>
        </div>
        <button
            class= "col-span-6 rounded bg-zinc-800 gap-2"
            on:click= move |_| {
                rw_context::<ChgSlot>(cx).update(|chg| chg.0 = eq_slot);
                ModalState::open(cx, 0);
            }
        >
            { item }
        </button>
    }
}

/// Fliters the inventory by the `slot` given and give the associated
/// original inventory index.
fn filter_by_slot(pc: &PC, slot: EquipSlot) -> Vec<(usize, Item)> {
    pc.inventory
        .iter()
        .enumerate()
        .filter(|(_, x)| {
            let spec = &x.spec;
            match slot {
                EquipSlot::Head => matches!(spec, ItemSpec::Head(_)),
                EquipSlot::MainHand => {
                    let has_off_hand = pc.get_equipment(EquipSlot::OffHand).is_some();
                    let can_hold = match spec {
                        ItemSpec::Weapon(_) => true,
                        ItemSpec::Tome(_) => true,
                        ItemSpec::Armour(x) => x.body_part as u8 == BodyPart::Held as u8,
                        _ => false,
                    };
                    if has_off_hand {
                        can_hold && x.weight < 2
                    } else {
                        can_hold
                    }
                }
                EquipSlot::OffHand => {
                    let is_held = match spec {
                        ItemSpec::Weapon(_) => true,
                        ItemSpec::Armour(x) => x.body_part as u8 == BodyPart::Held as u8,
                        _ => false,
                    };
                    is_held && x.weight < 2
                }
                EquipSlot::Body => match spec {
                    ItemSpec::Armour(x) => x.body_part as u8 == BodyPart::Body as u8,
                    _ => false,
                },
                EquipSlot::Legs => match spec {
                    ItemSpec::Armour(x) => x.body_part as u8 == BodyPart::Legs as u8,
                    _ => false,
                },
                EquipSlot::Tools => matches!(spec, ItemSpec::Tool),
            }
        })
        .map(|(i, x)| (i, x.clone()))
        .collect()
}

fn set_slot(cx: Scope, maybe_i: Option<usize>) {
    let slot = rw_context::<ChgSlot>(cx).with(|x| x.0);
    rw_context::<PC>(cx).update(|pc| {
        // Removing enhancements from removed item and then
        // adding back into inventory
        if let Some(item) = &pc.equipment[slot.index()] {
            session::rm_equipment(cx, item);
            pc.inventory.push(item.clone());
        };
        pc.equipment[slot.index()] = maybe_i.map(|i| pc.inventory.remove(i));
        // Adding enhancements from new item
        if let Some(item) = &pc.equipment[slot.index()] {
            session::add_equipment(cx, item);
        };
    });
    ModalState::dismiss(cx)
}

#[component]
fn ChangeEqModal(cx: Scope) -> impl IntoView {
    const BTN_CSS: &str = "rounded bg-zinc-700 py-2";
    let title = move || rw_context::<ChgSlot>(cx).with(|x| x.0);
    let item_view = move || {
        rw_context::<PC>(cx)
            .with_untracked(|pc| filter_by_slot(pc, title()))
            .into_iter()
            .map(|(i, item)| {
                view! {
                    cx,
                    <button
                        class=BTN_CSS
                        on:click=move |_| set_slot(cx, Some(i))
                    >
                        { item.into_view(cx) }
                    </button>
                }
            })
            .collect_view(cx)
    };

    view! {
        cx,
        <ModalCentered title id=0>
            <div class= "flex flex-col gap-2">
                { item_view }
                <button
                    class=concatcp!(BTN_CSS, " px-2 text-start")
                    on:click=move |_| set_slot(cx, None)
                >
                    "None"
                </button>
            </div>
        </ModalCentered>
    }
}
