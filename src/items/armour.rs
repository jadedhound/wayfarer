use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use super::enhancement::{self as enh, Enhancement};
use super::GearType;

#[derive(Serialize, Deserialize, Clone, EnumString, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Armour {
    // Body
    Robe,
    Gambeson,
    Brigandine,
    // Legs
    Leggings,
    Chausses,
    Greaves,
}

impl GearType for Armour {
    fn weight(&self) -> u8 {
        match self {
            Armour::Gambeson => 2,
            Armour::Brigandine => 2,
            Armour::Greaves => 2,
            _ => 1,
        }
    }

    fn enhancements(&self) -> Vec<Enhancement> {
        match self {
            Armour::Robe => vec![enh::stat::HP_1],
            Armour::Gambeson => vec![enh::stat::HP_1],
            Armour::Brigandine => vec![enh::stat::HP_1],
            Armour::Leggings => vec![enh::stat::HP_1],
            Armour::Chausses => vec![enh::stat::HP_1],
            Armour::Greaves => vec![enh::stat::HP_1],
        }
    }

    fn price(&self) -> u32 {
        let sp = match self {
            Armour::Robe | Armour::Gambeson | Armour::Brigandine => 500,
            Armour::Leggings | Armour::Chausses | Armour::Greaves => 250,
        };
        sp * 10
    }
}
