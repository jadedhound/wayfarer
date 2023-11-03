use serde::{Deserialize, Serialize};
use strum::Display;

use crate::buffs::{Buff, BuffRef};
use crate::pc::Ability;
use crate::utils::counter::Counter;

#[derive(Serialize, Deserialize, Clone, Display, PartialEq)]
pub enum ItemProp {
    Buff(Buff),
    Bulky(usize),
    Concentration,
    Count(Counter),
    Damage(usize),
    Effect(String),
    Passive,
    Range(u32),
    Resist,
    Score(Ability, i32),
    Usable(String),
}

#[derive(Clone, Copy, PartialEq)]
pub enum ItemPropRef {
    Buff(BuffRef),
    Bulky(usize),
    Concentration,
    Count(Counter),
    Damage(usize),
    Effect(&'static str),
    Passive,
    Range(u32),
    Resist,
    Score(Ability, i32),
    Usable(&'static str),
}

impl ItemProp {
    pub fn index(&self) -> usize {
        match self {
            ItemProp::Buff(_) => 1,
            ItemProp::Bulky(_) => 2,
            ItemProp::Concentration => 3,
            ItemProp::Count(_) => 4,
            ItemProp::Damage(_) => 5,
            ItemProp::Effect(_) => 6,
            ItemProp::Range(_) => 7,
            ItemProp::Resist => 8,
            ItemProp::Score(_, _) => 9,
            ItemProp::Usable(_) => 10,
            ItemProp::Passive => 11,
        }
    }
}

impl From<ItemPropRef> for ItemProp {
    fn from(value: ItemPropRef) -> Self {
        use ItemPropRef as Ref;

        match value {
            Ref::Bulky(x) => Self::Bulky(x),
            Ref::Resist => Self::Resist,
            Ref::Count(x) => Self::Count(x),
            Ref::Buff(x) => Self::Buff(x.into()),
            Ref::Usable(x) => Self::Usable(x.into()),
            Ref::Damage(x) => Self::Damage(x),
            Ref::Range(x) => Self::Range(x),
            Ref::Effect(x) => Self::Effect(x.into()),
            Ref::Passive => Self::Passive,
            Ref::Concentration => Self::Concentration,
            Ref::Score(x, y) => Self::Score(x, y),
        }
    }
}
