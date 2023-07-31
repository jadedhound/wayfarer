use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Feature {
    pub name: String,
    pub effect: String,
}

impl From<FeatureRef> for Feature {
    fn from(value: FeatureRef) -> Self {
        Self {
            name: value.name.into(),
            effect: value.effect.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct FeatureRef {
    pub name: &'static str,
    pub effect: &'static str,
}

impl FeatureRef {
    pub const fn new(name: &'static str, effect: &'static str) -> Self {
        Self { name, effect }
    }
}

pub const SEARING_STRIKES: FeatureRef =
    FeatureRef::new("searing strikes", "attacks deal fire damage");
pub const NIMBLE_ESCAPE: FeatureRef = FeatureRef::new(
    "nimble escape",
    "melee combat does not impede movement speed",
);
