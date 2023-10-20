use array_concat::{concat_arrays, concat_arrays_size};
pub use item_prop::*;
use serde::{Deserialize, Serialize};

use self::meta::ERROR_ITEM;
use crate::buffs::{Buff, BuffRef};
use crate::utils::counter::Counter;

pub mod adventure;
pub mod alchemist;
pub mod arcane;
pub mod blacksmith;
pub mod divine;
pub mod food;
mod item_prop;
pub mod meta;
pub mod search;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub props: Vec<ItemProp>,
    pub base_price: u32,
    pub desc: String,
}

impl Item {
    /// The price of an object is its base `item.price` x its count.
    pub fn price(&self) -> u32 {
        self.find_counter()
            .map(|count| count.curr as u32 * self.base_price)
            .unwrap_or(self.base_price)
    }
    pub fn is_bulky(&self) -> bool {
        self.props.contains(&ItemProp::Bulky)
    }
    pub fn find_mut_counter(&mut self) -> Option<&mut Counter> {
        self.props.iter_mut().find_map(|x| match x {
            ItemProp::Count(x) => Some(x),
            _ => None,
        })
    }
    pub fn find_counter(&self) -> Option<&Counter> {
        self.props.iter().find_map(|x| match x {
            ItemProp::Count(x) => Some(x),
            _ => None,
        })
    }
    /// Finds the first buff of the item.
    pub fn find_buff(&self) -> Option<&Buff> {
        self.props.iter().find_map(|x| match x {
            ItemProp::Buff(x) => Some(x),
            _ => None,
        })
    }
    /// Finds the first weapon damage of the item.
    pub fn find_damage(&self) -> Option<usize> {
        self.props.iter().find_map(|x| match x {
            ItemProp::Damage(x) => Some(*x),
            _ => None,
        })
    }
}

/// Static components of an item.
#[derive(Clone, Copy)]
pub struct ItemRef {
    pub name: &'static str,
    pub props: &'static [ItemPropRef],
    price: u32,
}

impl ItemRef {
    pub const fn new(name: &'static str, price: u32, props: &'static [ItemPropRef]) -> Self {
        Self { name, props, price }
    }
    pub const fn simple(name: &'static str, price: u32) -> Self {
        Self {
            name,
            props: &[],
            price,
        }
    }
    /// The price of an object is its base `item.price` * its count.
    pub fn price(&self) -> u32 {
        self.find_counter()
            .map(|count| count.curr as u32 * self.price)
            .unwrap_or(self.price)
    }
    pub fn find_counter(&self) -> Option<&Counter> {
        self.props.iter().find_map(|x| match x {
            ItemPropRef::Count(x) => Some(x),
            _ => None,
        })
    }
    pub fn is_bulky(&self) -> bool {
        self.props.contains(&ItemPropRef::Bulky)
    }
}

/// Possible damage dice which is a range of max base damage + quality range.
pub fn damage_die(i: usize) -> &'static str {
    pub const DAMAGE_DIE: [&str; 10] = [
        "1", "1d4", "1d6", "1d8", "1d10", "1d12", "2d6", "2d8", "2d10", "2d12",
    ];
    DAMAGE_DIE[std::cmp::min(i, DAMAGE_DIE.len() - 1)]
}

pub const BUFFS: [&BuffRef;
    concat_arrays_size!(
        adventure::BUFFS,
        alchemist::BUFFS,
        alchemist::BUFFS,
        arcane::BUFFS,
        divine::BUFFS,
        divine::BUFFS
    )] = concat_arrays!(
    adventure::BUFFS,
    alchemist::BUFFS,
    alchemist::BUFFS,
    arcane::BUFFS,
    divine::BUFFS,
    divine::BUFFS
);

// STARTER GEAR
pub const FIGHTER: [&ItemRef; 3] = [
    &adventure::TORCH,
    &blacksmith::WARHAMMER,
    &blacksmith::SHIELD,
];
pub const ROGUE: [&ItemRef; 3] = [
    &adventure::TORCH,
    &blacksmith::DAGGER,
    &blacksmith::LONGSWORD,
];
pub const MAGE: [&ItemRef; 3] = [
    &adventure::TORCH,
    &arcane::ARCANE_ARROW,
    &arcane::FEATHER_FALL,
];
pub const CLERIC: [&ItemRef; 3] = [&adventure::TORCH, &divine::MESSAGE, &divine::CHARM];

// -----------------------------------
// OTHER IMPL
// -----------------------------------

impl Default for Item {
    fn default() -> Self {
        ERROR_ITEM.into()
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.props.len() == other.props.len()
    }
}

impl From<ItemRef> for Item {
    fn from(value: ItemRef) -> Self {
        Self {
            name: value.name.into(),
            props: value.props.iter().map(|&x| x.into()).collect(),
            base_price: value.price,
            desc: String::new(),
        }
    }
}

impl Default for &'static ItemRef {
    fn default() -> Self {
        &ERROR_ITEM
    }
}
