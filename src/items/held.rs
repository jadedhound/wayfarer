use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use super::enhancement::{self as enh, Enhancement};
use super::GearType;
use crate::utils::{char_count_tuple, CharHash};

pub static HELD_TYPES: Lazy<HashMap<String, CharHash>> =
    Lazy::new(|| Held::iter().map(char_count_tuple).collect());

#[derive(Serialize, Deserialize, Clone, EnumString, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Held {
    Sword,
    Crowsbeak,
    Warhammer,
    Dagger,
    Axe,
    Shield,
    Spellbook,
    Prayerbook,
    Bow,
}

impl GearType for Held {
    fn weight(&self) -> u8 {
        match self {
            Held::Crowsbeak => 2,
            Held::Warhammer => 2,
            Held::Spellbook => 2,
            Held::Prayerbook => 2,
            Held::Bow => 2,
            _ => 1,
        }
    }

    fn enhancements(&self) -> Vec<Enhancement> {
        match self {
            Held::Sword => vec![enh::stat::DAMAGE_1],
            Held::Crowsbeak => vec![enh::stat::DAMAGE_1],
            Held::Warhammer => vec![enh::stat::DAMAGE_1],
            Held::Dagger => vec![enh::stat::DAMAGE_1],
            Held::Axe => vec![enh::stat::DAMAGE_1],
            Held::Shield => vec![enh::stat::DAMAGE_1],
            Held::Spellbook => vec![enh::stat::DAMAGE_1],
            Held::Prayerbook => vec![enh::stat::DAMAGE_1],
            Held::Bow => vec![enh::stat::DAMAGE_1],
        }
    }
}
