use serde::{Deserialize, Serialize};

use super::features::{Feature, FeatureRef};
use super::StatArr;

#[derive(Serialize, Deserialize, Clone)]
pub enum FeatOrStat {
    Feat(Feature),
    Stat(StatArr),
}

impl From<FeatOrStatRef> for FeatOrStat {
    fn from(value: FeatOrStatRef) -> Self {
        match value {
            FeatOrStatRef::Feat(x) => FeatOrStat::Feat(x.into()),
            FeatOrStatRef::Stat(x) => FeatOrStat::Stat(x),
        }
    }
}

#[derive(Clone, Copy)]
pub enum FeatOrStatRef {
    Feat(FeatureRef),
    Stat(StatArr),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Buff {
    pub name: String,
    pub duration: u8,
    pub effect: FeatOrStat,
}

impl PartialEq for Buff {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.duration == other.duration
    }
}

impl From<BuffRef> for Buff {
    fn from(value: BuffRef) -> Self {
        Self {
            name: value.name.into(),
            duration: value.duration,
            effect: value.effect.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct BuffRef {
    pub name: &'static str,
    pub duration: u8,
    pub effect: FeatOrStatRef,
}
