use std::write;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub mod buffs;
pub mod consumables;
pub mod effects;
pub mod food;
pub mod item_spec;
pub mod potions;
pub mod reagents;
pub mod recipes;
pub mod search;
pub mod simple;
pub mod tome;
pub mod weapons;

use strum::{Display, EnumCount, FromRepr, IntoEnumIterator};

use self::item_spec::{ItemSpec, ItemSpecRef};
use self::simple::ERROR_ITEM;
use crate::pc::PCStat;
use crate::utils::split_operator;

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
    pub weight: u8,
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
            weight: value.weight,
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
    weight: u8,
    price: u32,
    quality: ItemQuality,
    stacks: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct StatArr([i32; PCStat::COUNT]);

#[allow(dead_code)]
impl StatArr {
    pub fn iter(&self) -> impl Iterator<Item = &i32> + '_ {
        self.0.iter()
    }

    pub fn iter_with_stat(&self) -> impl Iterator<Item = (PCStat, &i32)> + '_ {
        PCStat::iter().zip(self.0.iter())
    }

    const fn new() -> Self {
        Self([0; PCStat::COUNT])
    }

    const fn hp(mut self, x: i32) -> Self {
        self.0[PCStat::HP.index()] += x;
        self
    }
    const fn str(mut self, x: i32) -> Self {
        self.0[PCStat::STR.index()] += x;
        self
    }
    const fn dex(mut self, x: i32) -> Self {
        self.0[PCStat::DEX.index()] += x;
        self
    }
    const fn int(mut self, x: i32) -> Self {
        self.0[PCStat::INT.index()] += x;
        self
    }
    const fn cha(mut self, x: i32) -> Self {
        self.0[PCStat::CHA.index()] += x;
        self
    }
}

/// Adjusts a given base price by quality. All items
/// scale exponentially from the base price.
const fn adj_price(base: u32, quality: ItemQuality) -> u32 {
    base * 2_u32.pow(quality as u32)
}

impl std::fmt::Display for StatArr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result =
            PCStat::iter()
                .zip(self.0.iter())
                .fold(String::new(), |mut acc, (stat, num)| {
                    if *num != 0 {
                        let (op, num) = split_operator(*num);
                        acc.push_str(&format!(", {stat} {op}{num}"))
                    }
                    acc
                });
        write!(f, "{result}")
    }
}
