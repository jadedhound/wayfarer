use super::{ItemPropRef as Prop, ItemRef};

pub const DAGGER: ItemRef = ItemRef::new("dagger", 5, &[Prop::Damage(1), Prop::Range(30)]);
pub const HANDAXE: ItemRef = ItemRef::new("handaxe", 5, &[Prop::Damage(1), Prop::Range(30)]);
pub const STAFF: ItemRef = ItemRef::new("staff", 5, &[Prop::Damage(1)]);

pub const BATTLE_AXE: ItemRef = ItemRef::new("battle axe", 10, &[Prop::Damage(2)]);
pub const SWORD: ItemRef = ItemRef::new("sword", 10, &[Prop::Damage(2)]);
pub const WARHAMMER: ItemRef = ItemRef::new("warhammer", 10, &[Prop::Damage(2)]);

pub const HALBERD: ItemRef = ItemRef::new("halberd", 30, &[Prop::Damage(3), Prop::Bulky]);
pub const ZWEIHANDER: ItemRef = ItemRef::new("zweihander", 30, &[Prop::Damage(3), Prop::Bulky]);

pub const LONGBOW: ItemRef = ItemRef::new(
    "longbow",
    30,
    &[Prop::Damage(2), Prop::Range(120), Prop::Bulky],
);
pub const CROSSBOW: ItemRef = ItemRef::new(
    "crossbow",
    60,
    &[
        Prop::Damage(3),
        Prop::Range(120),
        Prop::Effect("requires a movement or action to reload"),
        Prop::Bulky,
    ],
);
pub const ITEMS: [&ItemRef; 10] = [
    &DAGGER,
    &HANDAXE,
    &STAFF,
    &BATTLE_AXE,
    &SWORD,
    &WARHAMMER,
    &HALBERD,
    &ZWEIHANDER,
    &CROSSBOW,
    &LONGBOW,
];
