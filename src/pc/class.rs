use core::fmt;

use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::buffs::BuffRef;
use crate::items::{self, ItemRef};

mod buffs;
pub mod level;
pub mod view;
mod view_optional;

#[derive(Serialize, Deserialize, Clone, Copy, AsRefStr, PartialEq, Default)]
pub enum PCClassRef {
    #[default]
    Fighter,
    Rogue,
    Mage,
    Cleric,
}

#[derive(Clone, Copy)]
pub struct PCClass {
    pub prof: &'static str,
    pub base_buffs: &'static [&'static BuffRef],
    pub optional_buffs: &'static [&'static BuffRef],
    pub guard_bonus: i32,
    pub starter_gear: &'static [&'static ItemRef],
    pub adventuring_gear: usize,
}

impl PCClass {
    pub const fn new(
        prof: &'static str,
        base_buffs: &'static [&'static BuffRef],
        optional_buffs: &'static [&'static BuffRef],
        guard_bonus: i32,
        starter_gear: &'static [&'static ItemRef],
        adventuring_gear: usize,
    ) -> Self {
        Self {
            prof,
            base_buffs,
            optional_buffs,
            guard_bonus,
            starter_gear,
            adventuring_gear,
        }
    }
}

pub const FIGHTER: PCClass = PCClass::new(
    "military, intimidation and endurance",
    &buffs::FIGHTER_MAIN,
    &buffs::FIGHTER_OPTIONAL,
    6,
    &[
        &items::weaponsmith::WARHAMMER,
        &items::armoursmith::KITE_SHIELD,
    ],
    2,
);
pub const ROGUE: PCClass = PCClass::new(
    "delicate tasks, stealth and deception",
    &buffs::ROGUE_MAIN,
    &buffs::ROGUE_OPTIONAL,
    3,
    &[&items::weaponsmith::DAGGER, &items::weaponsmith::SWORD],
    4,
);
pub const MAGE: PCClass = PCClass::new(
    "arcane, history and insight",
    &buffs::MAGE_MAIN,
    &buffs::MAGE_OPTIONAL,
    1,
    &[
        &items::weaponsmith::DAGGER,
        &items::arcane::ARCANE_ARROW,
        &items::arcane::FEATHER_FALL,
    ],
    1,
);
pub const CLERIC: PCClass = PCClass::new(
    "divine, medicine and religion",
    &buffs::CLERIC_MAIN,
    &buffs::CLERIC_OPTIONAL,
    4,
    &[
        &items::weaponsmith::WARHAMMER,
        &items::divine::MESSAGE,
        &items::divine::CHARM,
    ],
    1,
);

// -----------------------------------
// TRAITS
// -----------------------------------

impl fmt::Display for PCClassRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl std::ops::Deref for PCClassRef {
    type Target = PCClass;

    fn deref(&self) -> &Self::Target {
        match self {
            PCClassRef::Fighter => &FIGHTER,
            PCClassRef::Rogue => &ROGUE,
            PCClassRef::Mage => &MAGE,
            PCClassRef::Cleric => &CLERIC,
        }
    }
}
