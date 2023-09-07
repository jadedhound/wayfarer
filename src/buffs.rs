use serde::{Deserialize, Serialize};

use crate::items::ItemPropRef;
use crate::pc::pc_stat::PCStat;
use crate::utils::counter::Counter;
use crate::utils::turns::Turns;

pub mod body;
pub mod class;
pub mod conditions;
pub mod held;
pub mod search;

#[derive(Serialize, Deserialize, Clone)]
pub struct Buff {
    pub name: String,
    pub props: Vec<BuffProp>,
}

impl Buff {
    pub fn find_effect(&self) -> Option<&String> {
        self.props.iter().find_map(|props| match props {
            BuffProp::Effect(x) => Some(x),
            _ => None,
        })
    }

    pub fn find_expiry(&self) -> Option<&Turns> {
        self.props.iter().find_map(|props| match props {
            BuffProp::Expiry(x) => Some(x),
            _ => None,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BuffProp {
    Effect(String),
    Class,
    Rest,
    Rally,
    Rechargable,
    Count(Counter),
    Duration(Turns),
    Expiry(Turns),
    StatOverride(PCStat, i32),
    Debuff,
}

#[derive(Clone, Copy)]
pub struct BuffRef {
    pub name: &'static str,
    pub props: &'static [BuffPropRef],
}

impl BuffRef {
    pub const fn new(name: &'static str, props: &'static [BuffPropRef]) -> Self {
        Self { name, props }
    }
}

#[derive(Clone, Copy)]
pub enum BuffPropRef {
    Effect(&'static str),
    Class,
    Rest,
    Rally,
    Rechargable,
    Count(Counter),
    Duration(Turns),
    StatOverride(PCStat, i32),
    Debuff,
}

// -----------------------------------
// DERIVE TRAITS
// -----------------------------------

impl PartialEq for Buff {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl From<BuffRef> for Buff {
    fn from(value: BuffRef) -> Self {
        Self {
            name: value.name.into(),
            props: value.props.iter().map(|&x| x.into()).collect(),
        }
    }
}

impl From<BuffPropRef> for BuffProp {
    fn from(value: BuffPropRef) -> Self {
        use BuffPropRef as Ref;
        match value {
            Ref::Effect(x) => Self::Effect(x.into()),
            Ref::Class => Self::Class,
            Ref::Rest => Self::Rest,
            Ref::Rally => Self::Rally,
            Ref::Count(x) => Self::Count(x),
            Ref::Duration(x) => Self::Duration(x),
            Ref::Rechargable => Self::Rechargable,
            Ref::StatOverride(x, y) => Self::StatOverride(x, y),
            Ref::Debuff => Self::Debuff,
        }
    }
}

impl From<BuffRef> for ItemPropRef {
    fn from(value: BuffRef) -> Self {
        Self::Buff(value)
    }
}

impl BuffProp {
    /// Due to BuffProp not being a primitive enum, explicit casting is required.
    fn index(&self) -> usize {
        match self {
            BuffProp::Effect(_) => 0,
            BuffProp::Class => 1,
            BuffProp::Rest => 2,
            BuffProp::Rally => 3,
            BuffProp::Rechargable => 4,
            BuffProp::Count(_) => 5,
            BuffProp::Duration(_) => 6,
            BuffProp::Expiry(_) => 7,
            BuffProp::StatOverride(_, _) => 8,
            BuffProp::Debuff => 9,
        }
    }
}

impl PartialEq for BuffProp {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

// -----------------------------------
// META
// -----------------------------------

const ERROR: BuffRef = BuffRef::new("error", &[]);
