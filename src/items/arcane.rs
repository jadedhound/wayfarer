#![cfg_attr(rustfmt, rustfmt_skip)]
use array_concat::{concat_arrays_size, concat_arrays};

use super::{ItemPropRef as Prop, ItemRef};
use crate::buffs::{BuffPropRef as BuffProp, BuffRef as Buff};
use crate::utils::turns::Turns;

const fn spell(name: &'static str, props: &'static [Prop]) -> ItemRef {
    ItemRef::new(name, 0, props)
}

pub const LIGHT: ItemRef = spell("scroll: light", &[Prop::Buff(LIGHT_BUFF)]);
const LIGHT_BUFF: Buff = Buff::new(
    "arcane: light",
    &[BuffProp::Effect("an object you've touched sheds 30 ft. of light around itself"), BuffProp::Duration(Turns::hour())],
);
pub const MINOR_ILLUSION: ItemRef = spell("scroll: minor illusion", &[Prop::Buff(MINOR_ILLUSION_BUFF)]);
const MINOR_ILLUSION_BUFF: Buff = Buff::new(
    "arcane: minor illusion",
    &[BuffProp::Effect("perform a subtle auditory or visual illusion"), BuffProp::Duration(Turns::one())],
);
pub const ARCANE_ARROW: ItemRef = spell(
    "scroll: arcane arrow", 
    &[Prop::Usable("fire a magic bolt up to 100 ft, all creatures within the line of effect take 1d4 damage"), Prop::WildMagic(0)]
);

pub const T1: [&ItemRef; 3] = [&LIGHT, &MINOR_ILLUSION, &ARCANE_ARROW];
pub const ALL: [&ItemRef; concat_arrays_size!(T1)] = concat_arrays!(T1);
pub const BUFFS: [&Buff; 2] = [&LIGHT_BUFF, &MINOR_ILLUSION_BUFF];
