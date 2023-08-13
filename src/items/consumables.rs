use super::effects::EffectRef;
use super::item_spec::ItemSpecRef;
use super::{adj_price, ItemQuality, ItemRef};

const fn consume(
    name: &'static str,
    effect: EffectRef,
    price: u32,
    quality: ItemQuality,
    stacks: Option<u8>,
) -> ItemRef {
    ItemRef {
        name,
        specs: ItemSpecRef::Consumable(effect),
        weight: 1,
        price,
        quality,
        stacks,
    }
}

const fn bomb(name: &'static str, effect: EffectRef, quality: ItemQuality) -> ItemRef {
    const BOMB_CP: u32 = 25;
    consume(name, effect, adj_price(BOMB_CP, quality), quality, Some(5))
}

const fn blast(desc: &'static str) -> EffectRef {
    EffectRef {
        can_recharge: false,
        desc,
    }
}

// ------------------------------
// KNOCK DOWN BOMBS
// ------------------------------

const BOMB_KNOCKDOWN: ItemRef = bomb(
    "knockdown bomb",
    blast("creatures within 5 ft. of the bomb are knocked down"),
    ItemQuality::Common,
);
const BOMB_CONCUSSIVE: ItemRef = bomb(
    "concussive charge",
    blast("creatures within 10 ft. of the bomb are pushed 5 ft. away and knocked down"),
    ItemQuality::Uncommon,
);

pub(super) const ITEMS_BOMB: [&ItemRef; 2] = [&BOMB_CONCUSSIVE, &BOMB_KNOCKDOWN];
