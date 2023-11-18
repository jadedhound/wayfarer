use std::cmp;

pub use item_prop::*;
use serde::{Deserialize, Serialize};

use crate::utils::counter::Counter;

pub mod adventure;
pub mod alchemist;
pub mod arcane;
pub mod armoursmith;
pub mod divine;
pub mod fletcher;
pub mod food;
pub mod illicit_goods;
mod item_prop;
pub mod meta;
pub mod weaponsmith;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub props: Vec<ItemProp>,
    pub base_price: u32,
}

impl Item {
    /// The price of an object is its base `item.price` x its count.
    pub fn price(&self) -> u32 {
        self.find_counter()
            .map(|count| counter_price(self.base_price, count))
            .unwrap_or(self.base_price)
    }
    pub fn find_mut_counter(&mut self) -> Option<&mut Counter> {
        self.props.iter_mut().find_map(|x| match x {
            ItemProp::Count(x) => Some(x),
            _ => None,
        })
    }
    pub fn find_counter(&self) -> Option<Counter> {
        self.props.iter().find_map(|x| match x {
            ItemProp::Count(x) => Some(*x),
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
    /// Finds the first the slots used by the item.
    pub fn is_bulky(&self) -> bool {
        self.props
            .iter()
            .any(|prop| matches!(prop, ItemProp::Bulky))
    }
}

/// Static components of an item.
#[derive(Clone, Copy)]
pub struct ItemRef {
    pub name: &'static str,
    pub props: &'static [ItemPropRef],
    base_price: u32,
}

impl ItemRef {
    pub const fn new(name: &'static str, base_price: u32, props: &'static [ItemPropRef]) -> Self {
        Self {
            name,
            props,
            base_price,
        }
    }
    pub const fn simple(name: &'static str, base_price: u32) -> Self {
        Self {
            name,
            props: &[],
            base_price,
        }
    }
    /// The price of an object is its base `item.price` * its count.
    pub fn price(&self) -> u32 {
        self.find_counter()
            .map(|count| counter_price(self.base_price, *count))
            .unwrap_or(self.base_price)
    }
    pub fn find_counter(&self) -> Option<&Counter> {
        self.props.iter().find_map(|x| match x {
            ItemPropRef::Count(x) => Some(x),
            _ => None,
        })
    }
    /// Finds the first the slots used by the item.
    pub fn is_bulky(&self) -> bool {
        self.props
            .iter()
            .any(|prop| matches!(prop, ItemPropRef::Bulky))
    }
}

fn counter_price(base: u32, counter: Counter) -> u32 {
    let ceil = |num: u32, dim: u32| num / dim + (num % dim != 0) as u32;
    let adjusted = (base / counter.max as u32) * counter.curr as u32;
    let rounded = ceil(adjusted, 5) * 5;
    cmp::max(rounded, 1)
}

/// Possible damage dice which is a range of max base damage + quality range.
pub fn damage_die(i: usize) -> &'static str {
    pub const DAMAGE_DIE: [&str; 10] = [
        "1", "1d4", "1d6", "1d8", "1d10", "1d12", "2d6", "2d8", "2d10", "2d12",
    ];
    DAMAGE_DIE[std::cmp::min(i, DAMAGE_DIE.len() - 1)]
}

pub const ALL: [&[&ItemRef]; 10] = [
    &adventure::ITEMS,
    &alchemist::ITEMS,
    &arcane::ITEMS,
    &armoursmith::ITEMS,
    &divine::ITEMS,
    &food::ITEMS,
    &meta::ITEMS,
    &weaponsmith::ITEMS,
    &illicit_goods::ITEMS,
    &fletcher::ITEMS,
];

// -----------------------------------
// OTHER IMPL
// -----------------------------------

impl Default for Item {
    fn default() -> Self {
        meta::ERROR_ITEM.into()
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
            base_price: value.base_price,
        }
    }
}

impl Default for &'static ItemRef {
    fn default() -> Self {
        &meta::ERROR_ITEM
    }
}
