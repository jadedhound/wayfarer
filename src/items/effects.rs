use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Effect {
    pub can_recharge: bool,
    pub desc: String,
}

impl From<EffectRef> for Effect {
    fn from(value: EffectRef) -> Self {
        Self {
            can_recharge: value.can_recharge,
            desc: value.desc.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct EffectRef {
    pub can_recharge: bool,
    pub desc: &'static str,
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Effect { can_recharge, desc } = self;
        let recharge = if *can_recharge {
            " Recharges after a rest."
        } else {
            ""
        };
        write!(f, "{desc}.{recharge}")
    }
}
