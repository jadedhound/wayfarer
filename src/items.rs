use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub mod alchemy;
pub mod consumables;
pub mod food;
mod item_prop;
pub mod search;
pub mod simple;
pub mod tome;
pub mod weapons;

pub use item_prop::*;

use self::simple::meta::ERROR_ITEM;
use crate::buffs::Buff;
use crate::utils::counter::Counter;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub props: Vec<ItemProp>,
    pub price: u32,
}

impl Item {
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
    pub price: u32,
}

impl ItemRef {
    pub const fn new(name: &'static str, price: u32, props: &'static [ItemPropRef]) -> Self {
        Self { name, props, price }
    }
}

// -----------------------------------
// OTHER IMPL
// -----------------------------------

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
        self.name == other.name && self.props.len() == other.props.len()
    }
}

impl From<ItemRef> for Item {
    fn from(value: ItemRef) -> Self {
        Self {
            name: value.name.into(),
            props: value.props.iter().map(|&x| x.into()).collect(),
            price: value.price,
        }
    }
}
