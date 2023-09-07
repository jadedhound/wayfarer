use super::{ItemPropRef, ItemRef};
use crate::buffs::{body, held};
use crate::items::ItemPropRef as Prop;

const PRICES: [u32; 5] = [50, 100, 200, 400, 1000];

#[rustfmt::skip]
pub mod t1 {
    use super::*;

    const fn t1(name: &'static str, props: &'static [ItemPropRef]) -> ItemRef {
        ItemRef::new(name, PRICES[1], props)
    }

    const LONGSTRIDER: ItemRef = t1(
        "potion of longstrider",
        &[Prop::Buff(body::LONGSTRIDER)]
    );
    const BOTTLE_BREATH: ItemRef = t1(
        "bottled breath",
        &[Prop::Buff(body::LONGBREATH)]
    );
    const BOTTLE_GREASE: ItemRef = t1(
        "bottle of grease",
        &[Prop::Usable("a bottle of thick flammable slippery liquid; has enough to cover a 10 ft square area")]
    );
    const OIL_FIRE: ItemRef = t1(
        "burning blade oil",
        &[Prop::Buff(held::OIL_FIRE)]
    );
    const OIL_FROST: ItemRef = t1(
        "freezing blade oil",
        &[Prop::Buff(held::OIL_FROST)]
    );
    const OIL_POISON: ItemRef = t1(
        "poisoned blade oil",
        &[Prop::Buff(held::OIL_POISON)]
    );
    const OIL_HOLY: ItemRef = t1(
        "holy blade oil",
        &[Prop::Buff(held::OIL_HOLY)]
    );
    const OIL_LIGHTNING: ItemRef = t1(
        "lightning blade oil",
        &[Prop::Buff(held::OIL_LIGHTNING)]
    );

    pub const ALL: [&ItemRef; 8] = [
        &LONGSTRIDER,
        &BOTTLE_BREATH, &BOTTLE_GREASE,
        &OIL_FIRE, &OIL_FROST, &OIL_HOLY, &OIL_POISON, &OIL_LIGHTNING
    ];
}
