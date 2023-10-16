use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::buffs::{Buff, BuffRef};
use crate::utils::counter::Counter;

#[derive(Serialize, Deserialize, Clone, AsRefStr, PartialEq)]
pub enum ItemProp {
    Bulky,
    Count(Counter),
    Resist,
    Buff(Buff),
    Usable(String),
    Edible(usize),
    Damage(usize),
    Range(u32),
    Effect(String),
    WildMagic(u8),
}

#[derive(Clone, Copy)]
pub enum ItemPropRef {
    Bulky,
    Count(Counter),
    Resist,
    Buff(BuffRef),
    Usable(&'static str),
    Edible(usize),
    Damage(usize),
    Range(u32),
    Effect(&'static str),
    WildMagic(u8),
}

impl From<ItemPropRef> for ItemProp {
    fn from(value: ItemPropRef) -> Self {
        use ItemPropRef as Ref;

        match value {
            Ref::Bulky => Self::Bulky,
            Ref::Resist => Self::Resist,
            Ref::Count(x) => Self::Count(x),
            Ref::Buff(x) => Self::Buff(x.into()),
            Ref::Usable(x) => Self::Usable(x.into()),
            Ref::Edible(x) => Self::Edible(x),
            Ref::Damage(x) => Self::Damage(x),
            Ref::Range(x) => Self::Range(x),
            Ref::Effect(x) => Self::Effect(x.into()),
            Ref::WildMagic(x) => Self::WildMagic(x),
        }
    }
}
