use core::fmt;

use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::buffs::BuffRef;

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
}

impl PCClass {
    pub const fn new(
        prof: &'static str,
        base_buffs: &'static [&'static BuffRef],
        optional_buffs: &'static [&'static BuffRef],
        guard_bonus: i32,
    ) -> Self {
        Self {
            prof,
            base_buffs,
            optional_buffs,
            guard_bonus,
        }
    }
}

pub const FIGHTER: PCClass = PCClass::new(
    "military, intimidation and endurance",
    &buffs::FIGHTER_MAIN,
    &buffs::FIGHTER_OPTIONAL,
    6,
);
pub const ROGUE: PCClass = PCClass::new(
    "delicate tasks, stealth and deception",
    &buffs::ROGUE_MAIN,
    &buffs::ROGUE_OPTIONAL,
    3,
);
pub const MAGE: PCClass = PCClass::new(
    "arcane, history and insight",
    &buffs::MAGE_MAIN,
    &buffs::MAGE_OPTIONAL,
    1,
);
pub const CLERIC: PCClass = PCClass::new(
    "divine, medicine and religion",
    &buffs::CLERIC_MAIN,
    &buffs::CLERIC_OPTIONAL,
    4,
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
