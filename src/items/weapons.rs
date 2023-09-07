#![cfg_attr(rustfmt, rustfmt_skip)]
use super::{ItemPropRef as Prop, ItemRef};

/// Possible damage dice which is a range of max base damage + quality range.
pub fn damage_die(i: usize) -> &'static str {
    pub const DAMAGE_DIE: [&str; 10] = [
        "1", "1d4", "1d6", "1d8", "1d10", "1d12", "2d6", "2d8", "2d10", "2d12",
    ];
    DAMAGE_DIE[std::cmp::min(i, DAMAGE_DIE.len() - 1)]
}

pub const HANDAXE: ItemRef = ItemRef::new("handaxe", 10, &[Prop::Damage(1), Prop::Range(30)]);
pub const HAMMER: ItemRef = ItemRef::new("warhammer", 10, &[Prop::Damage(2)]);
pub const CROWSBEAK: ItemRef = ItemRef::new("crowsbeak", 20, &[Prop::Damage(3), Prop::Bulky]);
pub const LONGBOW: ItemRef = ItemRef::new("longbow", 10, &[Prop::Damage(2), Prop::Range(120)]);
pub const SHIELD: ItemRef = ItemRef::new("shield", 10, &[Prop::Effect("halve all physical damage taken")]);
pub const DAGGER: ItemRef = ItemRef::new("dagger", 5, &[Prop::Damage(1), Prop::Range(30)]);
pub const SWORD: ItemRef = ItemRef::new("sword", 10, &[Prop::Damage(2)]);
pub const CROSSBOW: ItemRef = ItemRef::new(
    "crossbow", 10, &[Prop::Damage(3), Prop::Range(120), 
    Prop::Effect("requires a movement or action to reload")]
);

pub const ALL: [&ItemRef; 8] = [
    &HANDAXE, &HAMMER, &CROWSBEAK, &SHIELD,
    &DAGGER, &SWORD, 
    &CROSSBOW, &LONGBOW 
];
