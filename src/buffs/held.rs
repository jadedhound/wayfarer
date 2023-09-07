#![cfg_attr(rustfmt, rustfmt_skip)]

use super::{BuffPropRef as Prop, BuffRef};
use crate::utils::turns::Turns;

pub const TORCH: BuffRef = BuffRef::new(
    "torch light",
    &[Prop::Effect("a 30 ft circle around the torch is well lit"), Prop::Duration(Turns::hour())],
);
pub const OIL_FIRE: BuffRef = BuffRef::new(
    "burning blade",
    &[Prop::Effect("your weapon attacks deal fire damage instead of physical"), Prop::Duration(Turns::one())],
);
pub const OIL_FROST: BuffRef = BuffRef::new(
    "freezing blade",
    &[Prop::Effect("your weapon attacks deal frost damage instead of physical"), Prop::Duration(Turns::one())],
);
pub const OIL_POISON: BuffRef = BuffRef::new(
    "poisoned blade",
    &[Prop::Effect("your weapon attacks deal poison damage instead of physical"), Prop::Duration(Turns::one())],
);
pub const OIL_LIGHTNING: BuffRef = BuffRef::new(
    "lightning blade",
    &[Prop::Effect("your weapon attacks deal lightning damage instead of physical"), Prop::Duration(Turns::one())],
);
pub const OIL_HOLY: BuffRef = BuffRef::new(
    "holy blade",
    &[Prop::Effect("your weapon attacks deal holy damage instead of physical"), Prop::Duration(Turns::one())],
);

pub const ALL: [&BuffRef; 6] = [
    &TORCH,
    &OIL_FIRE, &OIL_FROST, &OIL_HOLY, &OIL_LIGHTNING, &OIL_POISON
];
