use const_format::concatcp;

use super::buffs::{BuffRef, FeatOrStatRef};
use super::item_specs::ItemSpecRef;
use super::{ItemQuality as IQ, ItemRef, StatArr};

const fn potion(name: &'static str, price: u32, quality: IQ, stat: StatArr) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Potion(BuffRef {
            name,
            duration: 3,
            effect: FeatOrStatRef::Stat(stat),
        }),
        weight: 1,
        price,
        quality,
    }
}

const PRE: &str = "potion of the ";
pub const POT_SAGE: ItemRef = potion(
    concatcp!(PRE, "sage"),
    250,
    IQ::Uncommon,
    StatArr::new().sorc(2),
);
pub const POT_AWKND_SHRUB: ItemRef = potion(
    concatcp!(PRE, "awakened shrub"),
    250,
    IQ::Common,
    StatArr::new().sorc(1),
);

pub const ITEMS_POTION: [&ItemRef; 2] = [&POT_SAGE, &POT_AWKND_SHRUB];
