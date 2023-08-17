use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Effect {
    pub desc: String,
}

impl From<EffectRef> for Effect {
    fn from(value: EffectRef) -> Self {
        Self {
            desc: value.desc.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct EffectRef {
    pub desc: &'static str,
}

impl EffectRef {
    pub const fn new(desc: &'static str) -> Self {
        Self { desc }
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Effect { desc } = self;
        write!(f, "{desc}")
    }
}
