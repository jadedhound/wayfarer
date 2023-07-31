use const_format::concatcp;

use super::item_specs::{ItemSpecRef, TomeRef};
use super::{ItemQuality, ItemRef};
use crate::pc::PCStat;

pub const DC_BY_QUALITY: [u8; 5] = [0, 0, 10, 15, 20];
const SP_PRICE: [u32; 5] = [0, 0, 50, 150, 300];

const fn tome(
    name: &'static str,
    effect: &'static str,
    quality: ItemQuality,
    stat: PCStat,
) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Tome(TomeRef { stat, effect }),
        weight: 2,
        price: SP_PRICE[quality as usize],
        quality,
    }
}

const S_PRE: &str = "spellbook: ";
const fn spell(name: &'static str, effect: &'static str) -> ItemRef {
    tome(name, effect, ItemQuality::Rare, PCStat::INT)
}
pub const ADHERE: ItemRef = spell(concatcp!(S_PRE, "attract"), "2 objects are polarized, if they come within 10 ft. they are strongly attracted to each other as if magnetised");

pub const ITEMS_SPELLBOOK: [&ItemRef; 1] = [&ADHERE];
