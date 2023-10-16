#![cfg_attr(rustfmt, rustfmt_skip)]

use array_concat::{concat_arrays, concat_arrays_size};
use serde::{Deserialize, Serialize};

mod alchemy;
mod arcane;
mod consumables;
mod divine;
mod food;
mod item_prop;
mod search;
mod simple;
mod weapons;

pub use item_prop::*;

use self::simple::meta::ERROR_ITEM;
use crate::buffs::{Buff, BuffRef};
use crate::pc::PC;
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

impl Default for &'static ItemRef {
    fn default() -> Self {
        &ERROR_ITEM
    }
}

// -----------------------------------
// PUBLIC
// -----------------------------------

// RE-EXPORTS
pub use search::search;
pub use simple::meta::FATIGUE;
pub use weapons::damage_die;

// SHOP LISTS
pub use alchemy::T1 as SHOP_ALCHEMY_T1;
pub use arcane::T1 as SHOP_ARCANE_T1;
pub use divine::T1 as SHOP_HOLY_T1;
pub use simple::sundry::ALL as SHOP_ADVENTURE_T1;
pub use weapons::ALL as SHOP_SMITH_T1;

// ARRAYS
pub const BUFFS: [&BuffRef; concat_arrays_size!(arcane::BUFFS, divine::BUFFS)] =
    concat_arrays!(arcane::BUFFS, divine::BUFFS);

// STARTER GEAR
pub const FIGHTER: [&ItemRef; 3] = [&consumables::TORCH, &weapons::WARHAMMER, &weapons::SHIELD];
pub const ROGUE: [&ItemRef; 3] = [&consumables::TORCH, &weapons::DAGGER, &weapons::SWORD];
pub const MAGE: [&ItemRef; 3] = [&consumables::TORCH, &arcane::LIGHT, &arcane::MINOR_ILLUSION];
pub const CLERIC: [&ItemRef; 3] = [&consumables::TORCH, &divine::MESSAGE, &divine::CHARM];
