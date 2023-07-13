use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use super::enhancement::{self as enh, Enhancement};
use super::GearType;
use crate::utils::{char_count_tuple, CharHash, LazyHash};

pub static ARMOUR_TYPES: LazyHash<CharHash> =
    Lazy::new(|| Armour::iter().map(char_count_tuple).collect());

#[derive(Serialize, Deserialize, Clone, EnumString, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Armour {
    // Body
    Robe,
    Gambeson,
    Brigandine,
    Plate,
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
            Armour::Plate => 3,
            Armour::Greaves => 2,
            _ => 1,
        }
    }

    fn enhancements(&self) -> Vec<Enhancement> {
        match self {
            Armour::Robe => vec![enh::stat::HP_1],
            Armour::Gambeson => vec![enh::stat::HP_1],
            Armour::Brigandine => vec![enh::stat::HP_1],
            Armour::Plate => vec![enh::stat::HP_1],
            Armour::Leggings => vec![enh::stat::HP_1],
            Armour::Chausses => vec![enh::stat::HP_1],
            Armour::Greaves => vec![enh::stat::HP_1],
        }
    }
}
