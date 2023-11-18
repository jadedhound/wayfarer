use super::{ItemPropRef as Prop, ItemRef};
use crate::pc::Ability;
use crate::utils::counter::Counter;

const ARMOUR_PROP: Prop = Prop::Usable("reduce physical damage by 2");

const BUCKLER: ItemRef = ItemRef::new("buckler", 10, &[ARMOUR_PROP]);
pub const KITE_SHIELD: ItemRef = ItemRef::new(
    "kite shield",
    15,
    &[ARMOUR_PROP, Prop::Count(Counter::new(2))],
);
const TOWER_SHIELD: ItemRef = ItemRef::new(
    "tower shield",
    10,
    &[ARMOUR_PROP, Prop::Count(Counter::new(4)), Prop::Bulky],
);

const GAMBESON: ItemRef = ItemRef::new("gambeson", 50, &[Prop::Passive, ARMOUR_PROP]);
const BRIGANDINE: ItemRef = ItemRef::new(
    "brigandine",
    150,
    &[
        Prop::Passive,
        ARMOUR_PROP,
        Prop::Count(Counter::new(3)),
        Prop::Bulky,
    ],
);
const CHAINMAIL: ItemRef = ItemRef::new(
    "chainmail",
    300,
    &[
        Prop::Passive,
        ARMOUR_PROP,
        Prop::Count(Counter::new(5)),
        Prop::Bulky,
    ],
);
const HALF_PLATE: ItemRef = ItemRef::new(
    "half plate",
    550,
    &[
        Prop::Passive,
        ARMOUR_PROP,
        Prop::Count(Counter::new(9)),
        Prop::Bulky,
    ],
);
const FULL_PLATE: ItemRef = ItemRef::new(
    "full plate",
    1000,
    &[
        Prop::Passive,
        ARMOUR_PROP,
        Prop::Count(Counter::new(12)),
        Prop::Bulky,
    ],
);

pub const ITEMS: [&ItemRef; 8] = [
    &BUCKLER,
    &KITE_SHIELD,
    &TOWER_SHIELD,
    &GAMBESON,
    &BRIGANDINE,
    &CHAINMAIL,
    &HALF_PLATE,
    &FULL_PLATE,
];
