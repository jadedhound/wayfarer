use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub mod buffs;
pub mod consumables;
pub mod effects;
pub mod food;
pub mod item_spec;
pub mod potions;
mod prices;
pub mod reagents;
pub mod recipes;
pub mod search;
pub mod simple;
pub mod tome;
pub mod weapons;

use strum::{Display, FromRepr};

use self::item_spec::{ItemSpec, ItemSpecRef};
use self::simple::ERROR_ITEM;

#[derive(Serialize, Deserialize, Copy, Clone, Display, FromRepr, Default)]
pub enum ItemQuality {
    #[default]
    Common,
    Uncommon,
    Rare,
    Wondrous,
    Mythical,
}

impl ItemQuality {
    pub fn colour(&self) -> &'static str {
        match self {
            ItemQuality::Common => "text-zinc-200",
            ItemQuality::Uncommon => "text-green-500",
            ItemQuality::Rare => "text-blue-500",
            ItemQuality::Wondrous => "text-purple-500",
            ItemQuality::Mythical => "text-orange-500",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub spec: ItemSpec,
    pub quality: ItemQuality,
    pub is_bulky: bool,
    pub price: u32,
    pub stacks: Option<(u8, u8)>,
}

impl Default for Item {
    fn default() -> Self {
        ERROR_ITEM.into()
    }
}

static EITEM: Lazy<Item> = Lazy::new(|| ERROR_ITEM.into());

impl Default for &Item {
    fn default() -> Self {
        &EITEM
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.quality as usize == other.quality as usize
    }
}

impl From<ItemRef> for Item {
    fn from(value: ItemRef) -> Self {
        Self {
            name: value.name.into(),
            spec: value.specs.into(),
            is_bulky: value.is_bulky,
            price: value.price,
            quality: value.quality,
            stacks: value.stacks.map(|x| (1, x)),
        }
    }
}

/// Static components of an item.
#[derive(Clone, Copy)]
pub struct ItemRef {
    name: &'static str,
    specs: ItemSpecRef,
    is_bulky: bool,
    price: u32,
    quality: ItemQuality,
    stacks: Option<u8>,
}

/// Adjusts a given base price by quality. All items
/// scale exponentially from the base price.
const fn adj_price(base: u32, quality: ItemQuality) -> u32 {
    base * 2_u32.pow(quality as u32)
}
