use serde::{Deserialize, Serialize};

pub mod armour;
pub mod attachable;
pub mod buffs;
pub mod features;
pub mod item_specs;
pub mod potions;
pub mod reagents;
pub mod recipes;
pub mod search;
pub mod simple;
pub mod tome;
pub mod weapons;

use strum::{Display, EnumCount, FromRepr, IntoEnumIterator};

use self::item_specs::{ItemSpec, ItemSpecRef};
use self::simple::ERROR_ITEM;
use crate::pc::PCStat;
use crate::utils::add_operator;

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
    pub id: u32,
    pub name: String,
    pub spec: ItemSpec,
    pub quality: ItemQuality,
    pub weight: u8,
    pub price: u32,
    pub attached: Option<(u32, Box<Item>)>,
}

impl Default for Item {
    fn default() -> Self {
        ERROR_ITEM.into()
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
            id: 0,
            name: value.name.into(),
            spec: value.specs.into(),
            weight: value.weight,
            price: value.price,
            quality: value.quality,
            attached: None,
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
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct StatArr(pub [i32; PCStat::COUNT]);

#[allow(dead_code)]
impl StatArr {
    /// Formats any stats that aren't 0 into stat and number.
    /// E.g. HP +2.
    pub fn string_iter(&self) -> impl Iterator<Item = String> + '_ {
        PCStat::iter().zip(self.0.iter()).flat_map(|(stat, num)| {
            if *num != 0 {
                let num = add_operator(*num);
                Some(format!("{stat} {num}"))
            } else {
                None
            }
        })
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
    const fn sorc(mut self, x: i32) -> Self {
        self.0[PCStat::Sorcery.index()] += x;
        self
    }
}
