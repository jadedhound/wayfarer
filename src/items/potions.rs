use const_format::formatcp;

use crate::utils::time::Turns;

use super::buffs::BuffRef;
use super::effects::EffectRef;
use super::item_spec::ItemSpecRef;
use super::prices::POTIONS;
use super::{ItemQuality as IQ, ItemRef};
use crate::pc::pc_stat::{StatArrBuilder, StatArray};

const PRE_COMMON: &str = "lesser potion of";
const PRE_UNCOMMON: &str = "potion of";
const PRE_RARE: &str = "greater potion of";
const PRE_WONDROUS: &str = "superb potion of";

const fn potion(
    name: &'static str,
    quality: IQ,
    duration: Turns,
    stats: Option<StatArray>,
    effect: Option<EffectRef>,
) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Potion(BuffRef {
            name,
            duration,
            stats,
            effect,
        }),
        is_bulky: false,
        price: POTIONS[quality as usize],
        quality,
        stacks: None,
    }
}

const fn stat(name: &'static str, quality: IQ, stats: StatArrBuilder) -> ItemRef {
    potion(name, quality, Turns::new(6), Some(stats.build()), None)
}

const fn effect(name: &'static str, quality: IQ, effect: EffectRef) -> ItemRef {
    potion(name, quality, Turns::new(6), None, Some(effect))
}

#[rustfmt::skip]
const fn both(name: &'static str, quality: IQ, stats: StatArrBuilder, effect: EffectRef) -> ItemRef {
    potion(name, quality, Turns::new(6), Some(stats.build()), Some(effect))
}

#[rustfmt::skip]
pub mod stat_incr {
    use crate::pc::pc_stat::StatArrBuilder;

    use super::*;

    pub const INT_T1: ItemRef = stat(
        formatcp!("{PRE_COMMON} cunning"), IQ::Common, 
        StatArrBuilder::new().int(1)
    );
    pub const INT_T2: ItemRef = stat(
        formatcp!("{PRE_UNCOMMON} cunning"), IQ::Uncommon, 
        StatArrBuilder::new().int(2)
    );
    pub const INT_T3: ItemRef = stat(
        formatcp!("{PRE_RARE} cunning"), IQ::Rare, 
        StatArrBuilder::new().int(3)
    );
    pub const INT_T4: ItemRef = stat(
        formatcp!("{PRE_WONDROUS} cunning"), IQ::Wondrous, 
        StatArrBuilder::new().int(4)
    );
}

pub const ALL: [&ItemRef; 4] = [
    &stat_incr::INT_T1,
    &stat_incr::INT_T2,
    &stat_incr::INT_T3,
    &stat_incr::INT_T4,
];
