#![cfg_attr(rustfmt, rustfmt_skip)]

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

pub(super) const BOMB: [&ItemRef; 2] = [&KNOCKDOWN_T2, &KNOCKDOWN_T1];

// ------------------------------
// MISC
// ------------------------------

pub mod misc {
    use super::*;

    const TORCH: ItemRef = ItemRef::new("torch", 1, &[Prop::Count(Counter::empty(5)), Prop::Buff(held::TORCH)]);

    pub const ALL: [&ItemRef; 1] = [&TORCH];
}
