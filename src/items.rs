use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

mod armour;
pub mod enhancement;
mod gems_and_runes;
mod held;
mod simple_items;

pub use armour::*;
use enhancement::Enhancement;
pub use gems_and_runes::*;
pub use held::*;
pub use simple_items::*;

use crate::utils::concat_some_str;

/// Gives a word associated with the quality tier given,
/// measured from 0 (lowest) to 3 (highest).
pub fn quality_str(q: u8) -> String {
    match q {
        0 => "Standard",
        1 => "Fine",
        2 => "Superb",
        3 => "Flawless",
        _ => "Error",
    }
    .to_string()
}

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

    pub fn as_held(&self) -> Option<Gear<Held>> {
        match self {
            Item::Held(h) => Some(h.clone()),
            _ => None,
        }
    }
    pub fn as_armour(&self) -> Option<Gear<Armour>> {
        match self {
            Item::Armour(a) => Some(a.clone()),
            _ => None,
        }
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
    T: FromStr + Display,
{
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
        concat_some_str(
            vec![
                self.gem.as_ref().map(|g| g.prefix.clone()),
                Some(self.base.to_string()),
                self.rune.as_ref().map(|r| r.suffix.clone()),
            ],
            " ",
        )
    }
}

pub trait GearType {
    fn weight(&self) -> u8;
    fn enhancements(&self) -> Vec<Enhancement>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HeadItem {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SimpleItem {
    pub name: String,
    pub weight: u8,
    pub uses: Option<u8>,
}

impl SimpleItem {
    pub fn new(name: &'static str) -> Self {
        Self {
            name: name.to_string(),
            weight: 1,
            uses: None,
        }
    }

    pub fn consumable(name: String, uses: u8) -> Self {
        Self {
            name,
            weight: 1,
            uses: Some(uses),
        }
    }

    pub fn weight(mut self, weight: u8) -> Self {
        self.weight = weight;
        self
    }
}
