use serde::{Deserialize, Serialize};
use strum::Display;

use crate::pc::Ability;
use crate::utils::counter::Counter;
use crate::utils::turns::Turns;

#[derive(Serialize, Deserialize, Clone, Display, PartialEq)]
pub enum ItemProp {
    Bulky,
    Concentration,
    Count(Counter),
    Damage(usize),
    Duration(Turns),
    Effect(String),
    Passive,
    Range(u32),
    Resist,
    Score(Ability, i32),
    Usable(String),
}

#[derive(Clone, Copy, PartialEq)]
pub enum ItemPropRef {
    Bulky,
    Concentration,
    Count(Counter),
    Damage(usize),
    Duration(Turns),
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
            ItemProp::Bulky => 1,
            ItemProp::Concentration => 2,
            ItemProp::Count(_) => 3,
            ItemProp::Damage(_) => 4,
            ItemProp::Duration(_) => 5,
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
            Ref::Bulky => Self::Bulky,
            Ref::Resist => Self::Resist,
            Ref::Count(x) => Self::Count(x),
            Ref::Usable(x) => Self::Usable(x.into()),
            Ref::Damage(x) => Self::Damage(x),
            Ref::Duration(x) => Self::Duration(x),
            Ref::Range(x) => Self::Range(x),
            Ref::Effect(x) => Self::Effect(x.into()),
            Ref::Passive => Self::Passive,
            Ref::Concentration => Self::Concentration,
            Ref::Score(x, y) => Self::Score(x, y),
        }
    }
}
