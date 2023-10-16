#![cfg_attr(rustfmt, rustfmt_skip)]
use array_concat::{concat_arrays_size, concat_arrays};

use super::{ItemPropRef as Prop, ItemRef};
use crate::buffs::{BuffPropRef as BuffProp, BuffRef as Buff};
use crate::utils::turns::Turns;

const fn free(name: &'static str, props: &'static [Prop]) -> ItemRef {
    ItemRef::new(name, 0, props)
}

pub const MESSAGE: ItemRef = free("scroll: message", &[Prop::Usable("send a brief psychic message to a known target 600 ft")]);
pub const CHARM: ItemRef = free("scroll: charm", &[Prop::Buff(CHARM_BUFF), Prop::Resist]);
const CHARM_BUFF: Buff = Buff::new(
    "divine: charm",
    &[BuffProp::Effect("one creature in 30 ft is friendly towards you"), BuffProp::Duration(Turns::hour())],
);
pub const GUIDING_BOLT: ItemRef = free(
    "scroll: guiding bolt", 
    &[Prop::Usable("the next attack against the chosen target deals maximum damage"), Prop::WildMagic(0)]
);

pub const T1: [&ItemRef; 3] = [&MESSAGE, &CHARM, &GUIDING_BOLT];
pub const ALL: [&ItemRef; concat_arrays_size!(T1)] = concat_arrays!(T1);
pub const BUFFS: [&Buff; 1] = [&CHARM_BUFF];
