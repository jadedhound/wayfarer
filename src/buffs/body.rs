#![cfg_attr(rustfmt, rustfmt_skip)]

use super::{BuffPropRef as Prop, BuffRef};
use crate::utils::turns::Turns;

// -----------------------------------
// FACE
// -----------------------------------

pub const LONGBREATH: BuffRef = BuffRef::new(
    "long breath", 
    &[Prop::Effect("you don't need to breathe while you hold this breath"), Prop::Duration(Turns::hour())]
);

// -----------------------------------
// SPEED
// -----------------------------------

pub const LONGSTRIDER: BuffRef = BuffRef::new(
    "longstrider", 
    &[Prop::Effect("your speed is now 45 ft"), Prop::Duration(Turns::hour())]
);

pub(super) const ALL: [&BuffRef; 1] = [&LONGSTRIDER];
