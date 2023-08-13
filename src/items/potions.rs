use const_format::concatcp;

use super::buffs::BuffRef;
use super::item_spec::ItemSpecRef;
use super::{ItemQuality as IQ, ItemRef, StatArr};

const fn potion(name: &'static str, price: u32, quality: IQ, stats: StatArr) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Potion(BuffRef {
            name,
            duration: 3,
            stats: Some(stats),
            effect: None,
        }),
        weight: 1,
        price,
        quality,
        stacks: None,
    }
}

const PRE: &str = "potion of the ";
pub const POT_SAGE: ItemRef = potion(
    concatcp!(PRE, "sage"),
    250,
    IQ::Uncommon,
    StatArr::new().cha(2),
);
pub const POT_AWKND_SHRUB: ItemRef = potion(
    concatcp!(PRE, "awakened shrub"),
    250,
    IQ::Common,
    StatArr::new().cha(1),
);

pub const ITEMS_POTION: [&ItemRef; 2] = [&POT_SAGE, &POT_AWKND_SHRUB];
