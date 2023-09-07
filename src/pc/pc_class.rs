use core::fmt;

use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::buffs::{
    class::{cleric, fighter, mage, rogue},
    BuffRef,
};

#[derive(Serialize, Deserialize, Clone, Copy, AsRefStr, PartialEq)]
pub enum PCClassRef {
    Fighter,
    Rogue,
    Mage,
    Cleric,
}

#[derive(Clone, Copy)]
pub struct PCClass {
    pub prof: &'static str,
    pub base_buffs: [&'static BuffRef; 3],
    pub optional_buffs: &'static [&'static BuffRef],
}

impl PCClass {
    pub const fn new(
        prof: &'static str,
        base_buffs: [&'static BuffRef; 3],
        optional_buffs: &'static [&'static BuffRef],
    ) -> Self {
        Self {
            prof,
            base_buffs,
            optional_buffs,
        }
    }
}

const FIGHTER_OPTIONAL: [&BuffRef; 3] = [&fighter::ENRAGE, &fighter::CHARGE, &fighter::ON_THE_HUNT];
pub const FIGHTER: PCClass = PCClass::new(
    "intimidation, endurance and military",
    [&fighter::T1, &fighter::T2, &fighter::T3],
    &FIGHTER_OPTIONAL,
);

const ROGUE_OPTIONAL: [&BuffRef; 2] = [&rogue::HUNTERS_MARK, &rogue::MAGE_HAND];
pub const ROGUE: PCClass = PCClass::new(
    "delicate tasks, stealth and deception",
    [&rogue::T1, &rogue::T2, &rogue::T3],
    &ROGUE_OPTIONAL,
);
const MAGE_OPTIONAL: [&BuffRef; 4] = [
    &mage::FIND_FAMILIAR,
    &mage::FIREBOLT,
    &mage::MAGECRAFT,
    &mage::METAMAGIC,
];
pub const MAGE: PCClass = PCClass::new(
    "arcane, history and insight",
    [&mage::T1, &mage::T2, &mage::T3],
    &MAGE_OPTIONAL,
);
const CLERIC_OPTIONAL: [&BuffRef; 3] = [
    &cleric::BULWARK_OF_FAITH,
    &cleric::SMITE,
    &cleric::TURN_UNDEAD,
];
pub const CLERIC: PCClass = PCClass::new(
    "divine, medicine and religion",
    [&cleric::T1, &cleric::T2, &cleric::T3],
    &CLERIC_OPTIONAL,
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
