#![cfg_attr(rustfmt, rustfmt_skip)]

use array_concat::{concat_arrays, concat_arrays_size};

use super::{ItemPropRef as Prop, ItemRef};
use crate::utils::counter::Counter;
use crate::buffs::held;


const fn use_prop(desc: &'static str) -> [Prop; 2] {
    [Prop::Count(Counter::full(5)), Prop::Usable(desc)]
}

// ------------------------------
// KNOCK DOWN BOMBS
// ------------------------------

const KNOCKDOWN_T1: ItemRef = ItemRef::new("lesser knockdown bomb", 25, &use_prop("creatures within 5 ft. of the bomb are knocked prone"));
const KNOCKDOWN_T2: ItemRef = ItemRef::new("knockdown bomb", 50, &use_prop("creatures within 10 ft. of the bomb are pushed away 5 ft. and knocked prone"));

pub const BOMB: [&ItemRef; 2] = [&KNOCKDOWN_T2, &KNOCKDOWN_T1];

pub const TORCH: ItemRef = ItemRef::new("torch", 5, &[Prop::Count(Counter::full(3)), Prop::Buff(held::TORCH)]);

pub const LIGHTING: [&ItemRef; 1] = [&TORCH];

pub const ALL: [&ItemRef; concat_arrays_size!(BOMB, LIGHTING)] = concat_arrays!(BOMB, LIGHTING);
