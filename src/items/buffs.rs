use serde::{Deserialize, Serialize};

use super::effects::{Effect, EffectRef};
use crate::pc::pc_stat::StatArray;
use crate::pc::PC;
use crate::utils::time::Turns;
use crate::utils::RwProvided;

#[derive(Serialize, Deserialize, Clone)]
pub struct Buff {
    pub name: String,
    pub duration: Turns,
    pub stats: Option<StatArray>,
    pub effect: Option<Effect>,
}

impl Buff {
    pub fn set_duration(&mut self) {
        self.duration.set(&PC::with(|pc| pc.turns));
    }
}

impl From<BuffRef> for Buff {
    fn from(value: BuffRef) -> Self {
        Self {
            name: value.name.into(),
            duration: value.duration,
            effect: value.effect.map(|x| x.into()),
            stats: value.stats,
        }
    }
}

impl std::fmt::Display for Buff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Buff {
            name,
            duration,
            stats,
            effect,
        } = self;
        let stats = stats.as_ref().map(|x| format!(" {x}.")).unwrap_or_default();
        let effect = effect
            .as_ref()
            .map(|x| format!(" {x}."))
            .unwrap_or_default();
        write!(f, "{name}:{stats}{effect} Lasts for {duration}.")
    }
}

#[derive(Clone, Copy)]
pub(super) struct BuffRef {
    pub name: &'static str,
    pub duration: Turns,
    pub stats: Option<StatArray>,
    pub effect: Option<EffectRef>,
}
