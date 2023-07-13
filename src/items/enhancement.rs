use std::matches;

use serde::{Deserialize, Serialize};

use crate::pc::PCStat;

#[derive(Serialize, Deserialize, Clone)]
pub struct StatIncrease {
    pub stat: PCStat,
    pub add: i32,
}

impl StatIncrease {
    pub const fn new(stat: PCStat, add: i32) -> Self {
        Self { stat, add }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Feature {
    pub name: String,
    pub effect: String,
    pub uses: u8,
}

impl Feature {
    pub fn new(name: &'static str, effect: &'static str, uses: u8) -> Self {
        Self {
            name: name.into(),
            effect: effect.into(),
            uses,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Enhancement {
    StatInc(StatIncrease),
    Feature(Feature),
}

impl Enhancement {
    pub fn is_feat(&self) -> bool {
        matches!(self, Enhancement::Feature(_))
    }
    pub fn is_stat(&self) -> bool {
        matches!(self, Enhancement::StatInc(_))
    }
}

pub mod stat {
    use super::{Enhancement as Enh, StatIncrease as Inc};
    use crate::pc::PCStat as Ps;

    pub const DAMAGE_1: Enh = Enh::StatInc(Inc::new(Ps::Damage, 1));
    pub const HP_1: Enh = Enh::StatInc(Inc::new(Ps::HP, 2));
}

pub mod feat {}
