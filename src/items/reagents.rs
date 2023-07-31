use super::item_specs::ItemSpecRef;
use super::{ItemQuality as IQ, ItemRef};

const fn reagent(name: &'static str, price: u32, quality: IQ) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Stackable(5),
        weight: 1,
        price,
        quality,
    }
}

// FORAGING
pub const FUNGI: ItemRef = reagent("fungi", 10, IQ::Common);
pub const SAGEROOT: ItemRef = reagent("sageroot", 10, IQ::Common);

// COMBAT
pub const ABERRANT_FLESH: ItemRef = reagent("aberrant flesh", 250, IQ::Rare);

pub const ITEMS_REAGENT: [&ItemRef; 3] = [&FUNGI, &SAGEROOT, &ABERRANT_FLESH];
