use serde::{Deserialize, Serialize};

use super::armour::{ArmourClass, BodyPart};
use super::features::{Feature, FeatureRef, NIMBLE_ESCAPE, SEARING_STRIKES};
use super::item_specs::ItemSpecRef;
use super::weapons::Weapon;
use super::{ItemQuality, ItemRef};

#[derive(Serialize, Deserialize, Clone)]
pub enum AttachFilter {
    Armour(ArmourClass, BodyPart),
    Weapon(Vec<Weapon>),
}

impl From<AttachFilterRef> for AttachFilter {
    fn from(value: AttachFilterRef) -> Self {
        match value {
            AttachFilterRef::Armour(x, y) => Self::Armour(x, y),
            AttachFilterRef::Weapon(x) => Self::Weapon(x.into()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum AttachFilterRef {
    Armour(ArmourClass, BodyPart),
    Weapon(&'static [Weapon]),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Attachable {
    filter: AttachFilter,
    feat: Feature,
}

impl From<AttachableRef> for Attachable {
    fn from(value: AttachableRef) -> Self {
        Self {
            filter: value.filter.into(),
            feat: value.feat.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct AttachableRef {
    filter: AttachFilterRef,
    feat: FeatureRef,
}

// FILTERS

const FILTER_BLADES: AttachFilterRef = AttachFilterRef::Weapon(&[Weapon::Sword, Weapon::Dagger]);
const FILTER_MEDIUM_LEGS: AttachFilterRef =
    AttachFilterRef::Armour(ArmourClass::Medium, BodyPart::Legs);

/// Prices for base attachables by quality level in silver pieces.
const SP_PRICES: [u32; 5] = [50, 100, 200, 400, 800];

/// Base attachable constructor.
const fn attach(
    name: &'static str,
    quality: ItemQuality,
    filter: AttachFilterRef,
    feat: FeatureRef,
) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Attachable(AttachableRef { filter, feat }),
        weight: 1,
        price: SP_PRICES[quality as usize] * 10,
        quality,
    }
}
const fn uncommon(name: &'static str, filter: AttachFilterRef, feat: FeatureRef) -> ItemRef {
    attach(name, ItemQuality::Uncommon, filter, feat)
}

// BLADE OILS
pub const SEARING_OIL: ItemRef = uncommon("searing oil", FILTER_BLADES, SEARING_STRIKES);

// MEDIUM ARMOUR
// LEGS

pub const NIMBLE_RUNE: ItemRef = uncommon("nimble rune", FILTER_MEDIUM_LEGS, NIMBLE_ESCAPE);

pub const ITEMS_ATTACH: [&ItemRef; 2] = [&SEARING_OIL, &NIMBLE_RUNE];
