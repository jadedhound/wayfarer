use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::buffs::{Buff, BuffRef};
use crate::utils::counter::Counter;

#[derive(Serialize, Deserialize, Clone, AsRefStr, PartialEq)]
pub enum ItemProp {
    Bulky,
    Count(Counter),
    Buff(Buff),
    Usable(String),
    Edible(usize),
    Spellbook(String),
    Damage(usize),
    Range(u32),
    Effect(String),
}

#[derive(Clone, Copy)]
pub enum ItemPropRef {
    Bulky,
    Count(Counter),
    Buff(BuffRef),
    Usable(&'static str),
    Edible(usize),
    Spellbook(&'static str),
    Damage(usize),
    Range(u32),
    Effect(&'static str),
}

impl From<ItemPropRef> for ItemProp {
    fn from(value: ItemPropRef) -> Self {
        match value {
            ItemPropRef::Bulky => Self::Bulky,
            ItemPropRef::Count(x) => Self::Count(x),
            ItemPropRef::Buff(x) => Self::Buff(x.into()),
            ItemPropRef::Usable(x) => Self::Usable(x.into()),
            ItemPropRef::Edible(x) => Self::Edible(x),
            ItemPropRef::Spellbook(x) => Self::Spellbook(x.into()),
            ItemPropRef::Damage(x) => Self::Damage(x),
            ItemPropRef::Range(x) => Self::Range(x),
            ItemPropRef::Effect(x) => Self::Effect(x.into()),
        }
    }
}
