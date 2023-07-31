use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, FromRepr};

use crate::items::armour::BodyPart;
use crate::items::item_specs::ItemSpec;
use crate::items::Item;

#[derive(Serialize, Deserialize, Clone, Copy, Display, EnumCount, EnumIter, FromRepr)]
pub(super) enum EquipSlot {
    Head,
    #[strum(serialize = "Main Hand")]
    MainHand,
    #[strum(serialize = "Off Hand")]
    OffHand,
    Body,
    Legs,
    #[strum(serialize = "Crafting Tools")]
    Tools,
}

impl EquipSlot {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

/// Takes a `Vec<Item>` and equips them to an appropriate slot.
///
/// Used to prefill a new PC's equipment. Selects items from the end of the
/// vec to the beginning.
pub(super) fn equip_items(inv: &mut Vec<Item>) -> [Option<Item>; EquipSlot::COUNT] {
    use EquipSlot as Slot;

    let mut arr = [0; Slot::COUNT].map(|_| None);
    let inv_clone = inv.clone();
    let mut assign = |arr: &mut [Option<Item>], slot: Slot, i, item: Item| {
        if arr[slot.index()].is_none() {
            arr[slot.index()] = Some(item);
            inv.remove(i);
        }
    };

    inv_clone
        .into_iter()
        .enumerate()
        // Remove indexes from the end to the start, to ensure
        // the correct item is being removed from the actual inventory
        .rev()
        .for_each(|(i, item)| match item.spec {
            ItemSpec::Head(_) => assign(&mut arr, Slot::Head, i, item),
            ItemSpec::Tome(_) => assign(&mut arr, Slot::MainHand, i, item),
            ItemSpec::Weapon(_) => {
                // Only equip large weapon if there is nothing is off hand
                if item.weight > 1 {
                    if arr[Slot::OffHand.index()].is_none() {
                        assign(&mut arr, Slot::MainHand, i, item)
                    }
                // Dual wield if already holding small weapon
                } else if arr[Slot::MainHand.index()]
                    .as_ref()
                    .is_some_and(|x| x.weight < 2)
                {
                    assign(&mut arr, Slot::OffHand, i, item)
                } else {
                    assign(&mut arr, Slot::MainHand, i, item)
                }
            }
            ItemSpec::Armour(x) => match x.body_part {
                BodyPart::Held => {
                    // Make sure off hand is empty AND main hand isn't holding a large weapon
                    if arr[Slot::OffHand.index()].is_none()
                        && arr[EquipSlot::OffHand.index()]
                            .as_ref()
                            .is_some_and(|x| x.weight < 2)
                    {
                        assign(&mut arr, Slot::OffHand, i, item)
                    }
                }
                BodyPart::Body => assign(&mut arr, Slot::Body, i, item),
                BodyPart::Legs => assign(&mut arr, Slot::Legs, i, item),
            },
            ItemSpec::Tool => assign(&mut arr, Slot::Tools, i, item),
            _ => (),
        });
    arr
}
