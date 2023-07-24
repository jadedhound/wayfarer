use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use super::enhancement::{self as enh, Enhancement};
use super::GearType;
use crate::pc::PCStat;

#[derive(Serialize, Deserialize, Clone, EnumString, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Held {
    // DEX
    Sword,    // Normal dmg
    Dagger,   // Low dmg, throwable
    Crossbow, // Ranged
    // STR
    Warhammer, // Normal dmg
    Handaxe,   // Low dmg, throwable
    Bow,       // Ranged
    Crowsbeak, // High dmg, two handed
    // OTHER
    Shield,
    Spellbook,
    Prayerbook,
}

impl Held {
    pub fn scale_by(&self) -> PCStat {
        match self {
            Held::Sword | Held::Dagger | Held::Crossbow => PCStat::DEX,
            _ => PCStat::STR,
        }
    }
}

impl GearType for Held {
    fn weight(&self) -> u8 {
        match self {
            Held::Crowsbeak => 2,
            Held::Bow => 2,
            Held::Crossbow => 2,
            Held::Spellbook => 2,
            Held::Prayerbook => 2,
            _ => 1,
        }
    }

    fn enhancements(&self) -> Vec<Enhancement> {
        match self {
            Held::Sword => vec![enh::stat::DAMAGE_2],
            Held::Crowsbeak => vec![enh::stat::DAMAGE_3],
            Held::Warhammer => vec![enh::stat::DAMAGE_2],
            Held::Dagger => vec![enh::stat::DAMAGE_1],
            Held::Handaxe => vec![enh::stat::DAMAGE_1],
            Held::Shield => vec![enh::stat::HP_1],
            Held::Spellbook => vec![],
            Held::Prayerbook => vec![],
            Held::Bow => vec![enh::stat::DAMAGE_2],
            Held::Crossbow => vec![enh::stat::DAMAGE_2],
        }
    }
    fn price(&self) -> u32 {
        let sp = match self {
            // DEX
            Held::Sword => 25,
            Held::Dagger => 10,
            Held::Crossbow => 100,
            // STR
            Held::Handaxe => 10,
            Held::Warhammer => 25,
            Held::Bow => 50,
            Held::Crowsbeak => 100,
            // OTHER
            Held::Shield => 10,
            Held::Spellbook | Held::Prayerbook => 0,
        };
        sp * 10
    }
}
