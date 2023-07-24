use std::cmp::min;

use strum::{Display, EnumIter};

use super::{Gear, GearType, Item, SimpleItem};

#[derive(Display, Clone, Copy)]
pub enum Artisan {
    Enchanter,
    Alchemist,
    Chef,
}

#[derive(Display, Clone, Copy, EnumIter)]
pub enum Experience {
    Novice,
    Seasoned,
    Expert,
    Master,
}

impl Experience {
    /// Each rank reduces time taken by 14 days.
    pub fn to_time(&self) -> u32 {
        (*self as u32 + 1) * 14
    }

    /// Each rank grants 5 additional DC reduction.
    pub fn to_dc(&self) -> u32 {
        (*self as u32 + 1) * 5
    }
}

pub fn enchanter_base(n: u32) -> u32 {
    2500 * 2_u32.pow(n)
}

fn upgrade_gear<T>(g: &Gear<T>) -> Gear<T>
where
    T: GearType + Clone,
{
    let mut g = g.clone();
    g.quality = min(g.quality + 1, 3);
    g
}

pub fn craft(artisan: Artisan, item: &Item) -> Item {
    let err = Item::Simple(SimpleItem::new("Error", 0));
    match artisan {
        Artisan::Enchanter => match item {
            Item::Held(h) => Item::Held(upgrade_gear(h)),
            Item::Armour(a) => Item::Armour(upgrade_gear(a)),
            _ => err,
        },
        Artisan::Alchemist => todo!(),
        Artisan::Chef => todo!(),
    }
}
