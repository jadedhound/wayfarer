use serde::{Deserialize, Serialize};

use super::item_specs::ItemSpecRef;
use super::{ItemQuality, ItemRef, StatArr};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum BodyPart {
    Held,
    Body,
    Legs,
}
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ArmourClass {
    Light,
    Medium,
    Heavy,
}
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Armour {
    pub body_part: BodyPart,
    pub class: ArmourClass,
    pub stats: StatArr,
}

const fn body(
    name: &'static str,
    weight: u8,
    price: u32,
    class: ArmourClass,
    stats: StatArr,
) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Armour(Armour {
            body_part: BodyPart::Body,
            class,
            stats,
        }),
        quality: ItemQuality::Common,
        weight,
        price,
    }
}

const fn legs(
    name: &'static str,
    weight: u8,
    price: u32,
    class: ArmourClass,
    stats: StatArr,
) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Armour(Armour {
            body_part: BodyPart::Legs,
            class,
            stats,
        }),
        quality: ItemQuality::Common,
        weight,
        price,
    }
}

const fn off_hand(name: &'static str, weight: u8, price: u32, stats: StatArr) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Armour(Armour {
            body_part: BodyPart::Held,
            class: ArmourClass::Medium,
            stats,
        }),
        quality: ItemQuality::Common,
        weight,
        price,
    }
}

// BODY WEAR
pub const ACO_ROBES: ItemRef = body(
    "acolyte's robes",
    1,
    500,
    ArmourClass::Light,
    StatArr::new().hp(1),
);
pub const MAGE_ROBE: ItemRef = body(
    "magician's robes",
    1,
    500,
    ArmourClass::Light,
    StatArr::new().hp(1),
);
const GAMBESON: ItemRef = body(
    "gambeson",
    2,
    500,
    ArmourClass::Medium,
    StatArr::new().hp(2),
);
pub const BRIGANDINE: ItemRef = body(
    "brigandine",
    2,
    500,
    ArmourClass::Heavy,
    StatArr::new().hp(3),
);

// LEG WEAR
pub const LEGGINGS: ItemRef = legs("leggings", 1, 250, ArmourClass::Light, StatArr::new().hp(1));
pub const CHAUSSES: ItemRef = legs(
    "chausses",
    1,
    250,
    ArmourClass::Medium,
    StatArr::new().hp(1),
);
const GREAVES: ItemRef = legs("greaves", 1, 250, ArmourClass::Heavy, StatArr::new().hp(2));

// OFF HAND
pub const SHIELD: ItemRef = off_hand("shield", 1, 250, StatArr::new().hp(2));

// STATIC ARRAYS
pub const ITEMS_HELD_ARMOUR: [&ItemRef; 1] = [&SHIELD];
pub const ITEMS_BODY: [&ItemRef; 4] = [&ACO_ROBES, &MAGE_ROBE, &GAMBESON, &BRIGANDINE];
pub const ITEMS_LEG: [&ItemRef; 3] = [&LEGGINGS, &CHAUSSES, &GREAVES];
