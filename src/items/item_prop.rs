use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::buffs::{Buff, BuffRef};
use crate::utils::counter::Counter;

#[derive(Serialize, Deserialize, Clone, AsRefStr, PartialEq)]
pub enum ItemProp {
    Buff(Buff),
    Bulky,
    Count(Counter),
    Concentration,
    Damage(usize),
    Effect(String),
    Food,
    Range(u32),
    Resist,
    Usable(String),
}

#[derive(Clone, Copy, PartialEq)]
pub enum ItemPropRef {
    Buff(BuffRef),
    Bulky,
    Count(Counter),
    Concentration,
    Damage(usize),
    Effect(&'static str),
    Food,
    Range(u32),
    Resist,
    Usable(&'static str),
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
            Ref::Food => Self::Food,
            Ref::Damage(x) => Self::Damage(x),
            Ref::Range(x) => Self::Range(x),
            Ref::Effect(x) => Self::Effect(x.into()),
            Ref::Concentration => Self::Concentration,
        }
    }
}
