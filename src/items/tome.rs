use const_format::formatcp;
use serde::{Deserialize, Serialize};

use super::{ItemPropRef as Prop, ItemRef};
use crate::pc::pc_stat::PCStat;

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

const fn tome(name: &'static str, props: &'static [Prop]) -> ItemRef {
    ItemRef::new(name, 250, props)
}

#[rustfmt::skip]
pub mod spell {
    use super::*;

    const PRE: &str = "spelltome of";
    const fn props(effect: &'static str) -> [Prop; 2] {
        [Prop::Bulky, Prop::Spellbook(effect)]
    }

    const ADHERE_PROP: [Prop; 2] = props("2 objects are polarized, if they come within 10 ft. they are strongly attracted");
    pub const ADHERE: ItemRef = tome(formatcp!("{PRE} attract"), &ADHERE_PROP);

    pub const ALL: [&ItemRef; 1] = [&ADHERE];
}

#[rustfmt::skip]
pub mod prayer {
    use super::*;

    const PRE: &str = "prayertome of";
    const fn props(effect: &'static str) -> [Prop; 2] {
        [Prop::Bulky, Prop::Spellbook(effect)]
    }

    const VENTRIL_PROP: [Prop; 2]= props("a creature must repeat everything you think; it is otherwise mute");
    pub const VENTRIL: ItemRef = tome(formatcp!("{PRE} the ventriloquist"), &VENTRIL_PROP);

    pub const ALL: [&ItemRef; 1] = [&VENTRIL];
}
