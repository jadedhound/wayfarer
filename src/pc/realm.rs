use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, FromRepr};

use crate::items::ItemRef;
use crate::utils::enum_array::{EnumArray, EnumRef};

mod followers;
mod overview;
mod rest;
mod sell;
mod shop;

pub use overview::realm;
pub use sell::Sell;
pub use shop::ShopView;

use super::session::Session;

#[derive(Serialize, Deserialize, Clone)]
pub struct Follower {
    pub name: String,
    pub level: u8,
    pub stats: FollowerStatArray,
}

#[derive(Serialize, Deserialize, Clone, Copy, EnumCount, EnumIter, Display)]
pub enum FollowerStat {
    Health,
    Expertise,
    Inventory,
    Morale,
}

impl EnumRef for FollowerStat {
    fn index(&self) -> usize {
        *self as usize
    }
}

const STAT_COUNT: usize = FollowerStat::COUNT;
pub type FollowerStatArray = EnumArray<FollowerStat, STAT_COUNT>;

#[derive(Clone, Copy, EnumIter, Display, FromRepr, Default)]
pub enum Shop {
    #[default]
    Alchemist,
    #[strum(serialize = "Adventuring Supplies")]
    Adventurer,
    Blacksmith,
    #[strum(serialize = "Arcane Forge")]
    Arcane,
    #[strum(serialize = "Hallowed Ground")]
    Divine,
}

impl Shop {
    fn items(&self) -> &[&'static ItemRef] {
        use crate::items::{adventure, alchemist, arcane, blacksmith, divine};
        match self {
            Shop::Alchemist => &alchemist::ITEMS,
            Shop::Blacksmith => &blacksmith::ITEMS,
            Shop::Adventurer => &adventure::ITEMS,
            Shop::Arcane => &arcane::ITEMS,
            Shop::Divine => &divine::ITEMS,
        }
    }

    /// If certain crafting requirements aren't met, the crafting area
    /// cannot be used by the PC.
    fn cannot_use(&self, sesh: &Session) -> bool {
        match self {
            Shop::Arcane => sesh.cast_arcane < 1,
            Shop::Divine => sesh.cast_divine < 1,
            _ => false,
        }
    }

    /// Flavourful description of the shop.
    fn desc(&self) -> &'static str {
        match self {
            Shop::Alchemist => "Liquids, powders and gases fill containers of all sizes, best step carefully around these shelves.",
            Shop::Adventurer => "The walls are covered with gear of all kinds, some of them not so new.",
            Shop::Blacksmith => "The sour smell of sweat, a tide of heat and the rythmic beats of a hammer assault your senses.",
            Shop::Arcane => "Components, mundane to most but not you, for you have gazed beyond the veil and see them for what they truly are.",
            Shop::Divine => "You step on holy ground, the gods are likely to listen to your requests for aid.",
        }
    }
}
