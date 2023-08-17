use serde::{Deserialize, Serialize};

use super::buffs::{Buff, BuffRef};
use super::item_spec::ItemSpecRef;
use super::{ItemQuality, ItemRef};

#[derive(Serialize, Deserialize, Clone)]
pub struct Food {
    pub buff: Option<Buff>,
    pub fatigue: u8,
}

impl From<FoodRef> for Food {
    fn from(value: FoodRef) -> Self {
        Self {
            buff: value.buff.map(|x| x.into()),
            fatigue: value.fatigue,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct FoodRef {
    buff: Option<BuffRef>,
    fatigue: u8,
}

impl FoodRef {
    const fn fatigue(fatigue: u8) -> Self {
        Self {
            buff: None,
            fatigue,
        }
    }
}

const fn food(name: &'static str, food: FoodRef, quality: ItemQuality) -> ItemRef {
    const PRICES: [u32; 5] = [1, 10, 25, 50, 100];
    ItemRef {
        name,
        specs: ItemSpecRef::Food(food),
        is_bulky: false,
        price: PRICES[quality as usize],
        quality,
        stacks: Some(4),
    }
}

const FOOD_TACK: ItemRef = food("hard tack", FoodRef::fatigue(1), ItemQuality::Common);

pub(super) const ALL: [&ItemRef; 1] = [&FOOD_TACK];
