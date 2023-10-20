use super::ItemRef;
use crate::buffs::{BuffPropRef as BuffProp, BuffRef};
use crate::items::ItemPropRef as Prop;
use crate::utils::turns::Turns;

const PRICES: [u32; 5] = [25, 50, 200, 400, 1000];

const fn tier_1(name: &'static str, props: &'static [Prop]) -> ItemRef {
    ItemRef::new(name, PRICES[0], props)
}

const LONGSTRIDER: ItemRef = tier_1("potion of longstrider", &[Prop::Buff(LONGSTRIDER_BUFF)]);
const LONGSTRIDER_BUFF: BuffRef = BuffRef::new(
    "longstrider",
    &[
        BuffProp::Effect("your speed is now 45 ft"),
        BuffProp::Duration(Turns::hour()),
    ],
);
const BOTTLE_BREATH: ItemRef = tier_1("bottled breath", &[Prop::Buff(LONGBREATH_BUFF)]);
const LONGBREATH_BUFF: BuffRef = BuffRef::new(
    "long breath",
    &[
        BuffProp::Effect("you don't need to breathe while you hold this breath"),
        BuffProp::Duration(Turns::hour()),
    ],
);
const BOTTLE_GREASE: ItemRef = tier_1(
    "bottle of grease",
    &[Prop::Usable(
        "a bottle of thick flammable slippery liquid; has enough to cover a 10 ft square area",
    )],
);

pub const BUFFS: [&BuffRef; 2] = [&LONGSTRIDER_BUFF, &LONGBREATH_BUFF];
pub const ITEMS: [&ItemRef; 3] = [&LONGSTRIDER, &BOTTLE_BREATH, &BOTTLE_GREASE];
