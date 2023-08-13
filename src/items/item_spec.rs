use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::buffs::{Buff, BuffRef};
use super::effects::{Effect, EffectRef};
use super::food::{Food, FoodRef};
use super::tome::{Tome, TomeRef};
use super::weapons::Weapon;
use super::StatArr;

#[derive(Serialize, Deserialize, Clone, AsRefStr)]
pub enum ItemSpec {
    Weapon(Weapon),
    Tome(Tome),
    Buff(Buff),
    Consumable(Effect),
    Food(Food),
    Simple,
}

impl ItemSpec {
    pub fn as_stat_arr(&self) -> Option<&StatArr> {
        None
    }

    pub fn as_weapon(&self) -> Option<&Weapon> {
        match self {
            ItemSpec::Weapon(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_food(&self) -> Option<&Food> {
        match self {
            ItemSpec::Food(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_effect(&self) -> Option<&Effect> {
        match self {
            ItemSpec::Consumable(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_mut_effect(&mut self) -> Option<&mut Effect> {
        match self {
            ItemSpec::Consumable(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_buff(&self) -> Option<&Buff> {
        match self {
            ItemSpec::Buff(x) => Some(x),
            _ => None,
        }
    }
}

impl From<ItemSpecRef> for ItemSpec {
    fn from(value: ItemSpecRef) -> Self {
        match value {
            ItemSpecRef::Weapon(x) => Self::Weapon(x),
            ItemSpecRef::Potion(x) => Self::Buff(x.into()),
            ItemSpecRef::Simple => Self::Simple,
            ItemSpecRef::Tome(x) => Self::Tome(x.into()),
            ItemSpecRef::Consumable(x) => Self::Consumable(x.into()),
            ItemSpecRef::Food(x) => Self::Food(x.into()),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum ItemSpecRef {
    Weapon(Weapon),
    Tome(TomeRef),
    Potion(BuffRef),
    Consumable(EffectRef),
    Food(FoodRef),
    Simple,
}
