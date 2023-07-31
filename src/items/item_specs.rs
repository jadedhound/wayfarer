use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::armour::Armour;
use super::attachable::{Attachable, AttachableRef};
use super::buffs::{Buff, BuffRef};
use super::features::{Feature, FeatureRef};
use super::weapons::Weapon;
use super::StatArr;
use crate::pc::PCStat;

#[derive(Serialize, Deserialize, Clone, AsRefStr)]
pub enum ItemSpec {
    Head(Feature),
    Weapon(Weapon),
    Tome(Tome),
    Armour(Armour),
    Buff(Buff),
    Stackable(u8, u8),
    Attachable(Attachable),
    Tool,
    Simple,
}

impl ItemSpec {
    pub fn as_stat_arr(&self) -> Option<&StatArr> {
        match self {
            ItemSpec::Armour(x) => Some(&x.stats),
            _ => None,
        }
    }

    pub fn as_feat(&self) -> Option<&Feature> {
        match self {
            ItemSpec::Head(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_weapon(&self) -> Option<&Weapon> {
        match self {
            ItemSpec::Weapon(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_stackable(&self) -> Option<(&u8, &u8)> {
        match self {
            ItemSpec::Stackable(x, y) => Some((x, y)),
            _ => None,
        }
    }
}

impl From<ItemSpecRef> for ItemSpec {
    fn from(value: ItemSpecRef) -> Self {
        match value {
            ItemSpecRef::Head(x) => Self::Head(x.into()),
            ItemSpecRef::Weapon(x) => Self::Weapon(x),
            ItemSpecRef::Armour(x) => Self::Armour(x),
            ItemSpecRef::Potion(x) => Self::Buff(x.into()),
            ItemSpecRef::Simple => Self::Simple,
            ItemSpecRef::Stackable(x) => Self::Stackable(1, x),
            ItemSpecRef::Tool => Self::Tool,
            ItemSpecRef::Tome(x) => Self::Tome(x.into()),
            ItemSpecRef::Attachable(x) => Self::Attachable(x.into()),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum ItemSpecRef {
    Head(FeatureRef),
    Weapon(Weapon),
    Tome(TomeRef),
    Armour(Armour),
    Potion(BuffRef),
    Stackable(u8),
    Attachable(AttachableRef),
    Tool,
    Simple,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tome {
    pub stat: PCStat,
    pub effect: String,
}

impl From<TomeRef> for Tome {
    fn from(value: TomeRef) -> Self {
        Self {
            stat: value.stat,
            effect: value.effect.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct TomeRef {
    pub stat: PCStat,
    pub effect: &'static str,
}
