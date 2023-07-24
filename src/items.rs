use std::fmt::Display;
use std::str::FromStr;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

mod armour;
pub mod craft;
pub mod enhancement;
mod fuzzy_match;
mod gems_and_runes;
mod held;
mod reagents;
mod search;
mod simple_items;

pub use armour::*;
use enhancement::Enhancement;
pub use gems_and_runes::*;
pub use held::*;
pub use reagents::*;
pub use search::search;
pub use simple_items::*;

use self::craft::enchanter_base;
use crate::utils::VecStrOps;

#[derive(Serialize, Deserialize, Clone)]
pub enum Item {
    Held(Gear<Held>),
    Head(HeadItem),
    Armour(Gear<Armour>),
    Simple(SimpleItem),
    Fatigue,
}

impl Item {
    pub fn name(&self) -> String {
        match self {
            Item::Held(a) => a.name(),
            Item::Armour(a) => a.name(),
            Item::Simple(a) => a.name.clone(),
            Item::Head(a) => a.name.clone(),
            Item::Fatigue => "Fatigue".into(),
        }
    }

    pub fn weight(&self) -> u8 {
        match self {
            Item::Held(a) => a.base.weight(),
            Item::Armour(a) => a.base.weight(),
            Item::Simple(a) => a.weight,
            Item::Head(_) => 1,
            Item::Fatigue => 1,
        }
    }

    pub fn price(&self) -> u32 {
        match self {
            Item::Held(a) => a.price(),
            Item::Armour(a) => a.price(),
            Item::Simple(a) => a.price,
            Item::Head(_) => 0,
            Item::Fatigue => 0,
        }
    }

    /// The quality rate of the item,
    /// ranging from lowest (0) to highest (3).
    pub fn quality(&self) -> u8 {
        match self {
            Item::Held(a) => a.quality,
            Item::Armour(a) => a.quality,
            Item::Head(_) => 3,
            Item::Simple(_) => todo!(),
            _ => 0,
        }
    }

    pub fn as_held(&self) -> Option<Gear<Held>> {
        match self {
            Item::Held(h) => Some(h.clone()),
            _ => None,
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::Simple(SimpleItem::new("Error", 0))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Gear<T> {
    pub quality: u8,
    pub base: T,
    pub gem: Option<Gem>,
    pub rune: Option<Rune>,
}

impl<T> Gear<T>
where
    T: FromStr + Display + GearType,
{
    pub fn price(&self) -> u32 {
        let n = self.quality as u32;
        // To ensure a standard item has the base price only
        self.base.price() + (enchanter_base(n) - enchanter_base(0))
    }

    pub fn create(
        gem: Option<String>,
        base: String,
        rune: Option<String>,
        quality: u8,
    ) -> Option<Self> {
        let gem = gem.as_ref().and_then(|n| GEMS.get(n).cloned());
        let rune = rune.as_ref().and_then(|n| RUNES.get(n).cloned());
        let base = T::from_str(&base).ok()?;
        Some(Self {
            quality,
            base,
            gem,
            rune,
        })
    }

    pub fn name(&self) -> String {
        vec![
            self.gem.as_ref().map(|g| g.prefix.clone()),
            Some(self.base.to_string()),
            self.rune.as_ref().map(|r| r.suffix.clone()),
        ]
        .flat_concat(" ")
        // Name is some, so will always succeed
        .unwrap()
    }

    /// Gives a word associated with the quality tier given,
    /// measured from 0 (lowest) to 3 (highest).
    pub fn quality_str(&self) -> &'static str {
        match self.quality {
            0 => "Standard",
            1 => "Fine",
            2 => "Superb",
            3 => "Flawless",
            _ => "Error",
        }
    }
}

pub trait GearType {
    fn weight(&self) -> u8;
    fn enhancements(&self) -> Vec<Enhancement>;
    fn price(&self) -> u32;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HeadItem {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SimpleItem {
    pub name: String,
    pub weight: u8,
    pub price: u32,
}

impl SimpleItem {
    pub fn new(name: &'static str, sp: u32) -> Self {
        Self {
            name: name.to_string(),
            weight: 1,
            price: sp * 10,
        }
    }

    pub fn weight(mut self, weight: u8) -> Self {
        self.weight = weight;
        self
    }
}
