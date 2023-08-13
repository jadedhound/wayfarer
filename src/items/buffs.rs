use leptos::*;
use serde::{Deserialize, Serialize};

use super::effects::{Effect, EffectRef};
use super::StatArr;
use crate::pc::PC;
use crate::utils::rw_context;

#[derive(Serialize, Deserialize, Clone)]
pub struct Buff {
    pub name: String,
    pub duration: u64,
    pub stats: Option<StatArr>,
    pub effect: Option<Effect>,
}

impl Buff {
    pub fn set_duration(mut self, cx: Scope) -> Self {
        let curr = rw_context::<PC>(cx).with(|pc| pc.turns);
        self.duration += curr;
        self
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
        let duration = if duration > &24 {
            format!("{duration} turns")
        } else {
            format!("{} days", duration / (24 * 60 * 6))
        };
        write!(f, "{name}:{stats}{effect} Lasts for {duration}.")
    }
}

#[derive(Clone, Copy)]
pub(super) struct BuffRef {
    pub name: &'static str,
    pub duration: u64,
    pub stats: Option<StatArr>,
    pub effect: Option<EffectRef>,
}
