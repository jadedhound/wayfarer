use serde::{Deserialize, Serialize};

use super::item_specs::ItemSpecRef;
use super::{ItemQuality, ItemRef};
use crate::pc::PCStat;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Weapon {
    // DEX
    Dagger,
    Sword,
    Crossbow,
    // STR
    Axe,
    Polearm,
    Bow,
}

impl Weapon {
    pub fn as_stat(&self) -> PCStat {
        match self {
            Self::Dagger | Self::Sword | Self::Crossbow => PCStat::DEX,
            _ => PCStat::STR,
        }
    }
    pub fn as_damage(&self) -> usize {
        match self {
            Self::Dagger | Self::Axe => 2,
            Self::Sword | Self::Bow => 3,
            Self::Crossbow | Self::Polearm => 4,
        }
    }
}

/// Possible damage dice which is a range of max base damage + quality range.
pub const DAMAGE_DIE: [&str; 8] = ["1", "1d4", "1d6", "1d8", "1d10", "1d12", "2d6", "2d8"];

const SP_PRICES: [u32; 5] = [25, 100, 200, 400, 800];

const fn weapons(name: &'static str, weapon: Weapon, quality: ItemQuality) -> ItemRef {
    let weight = match weapon {
        Weapon::Dagger | Weapon::Sword | Weapon::Axe => 1,
        _ => 2,
    };
    ItemRef {
        name,
        specs: ItemSpecRef::Weapon(weapon),
        weight,
        price: SP_PRICES[quality as usize] * 10 * weight as u32,
        quality: ItemQuality::Common,
    }
}

pub const SWORD: ItemRef = weapons("worn sword", Weapon::Sword, ItemQuality::Common);
pub const WARHAMMER: ItemRef = weapons("worn handaxe", Weapon::Axe, ItemQuality::Common);

pub const ITEMS_WEAP: [&ItemRef; 2] = [&SWORD, &WARHAMMER];
