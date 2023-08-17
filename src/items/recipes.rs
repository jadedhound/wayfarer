use serde::{Deserialize, Serialize};

use super::reagents::Substance;
use super::{Item, ItemRef};
pub use rcp::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub name: String,
    pub substances: [Substance; 2],
    pub products: [Item; 4],
}

impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl From<RecipeRef> for Recipe {
    fn from(value: RecipeRef) -> Self {
        Self {
            name: value.name.into(),
            substances: value.substances,
            products: value.products.map(|x| x.into()),
        }
    }
}

pub struct RecipeRef {
    pub name: &'static str,
    pub substances: [Substance; 2],
    pub products: [ItemRef; 4],
}

impl RecipeRef {
    pub const fn new(
        name: &'static str,
        substances: [Substance; 2],
        products: [ItemRef; 4],
    ) -> Self {
        Self {
            name,
            substances,
            products,
        }
    }
}

#[rustfmt::skip]
mod rcp {
    use super::*;
    use crate::items::potions::stat_incr;
    
    pub const CUNNING: RecipeRef = RecipeRef::new(
        "potion of cunning",
        [Substance::Solvent, Substance::Arcane], 
        [stat_incr::INT_T1, stat_incr::INT_T2, stat_incr::INT_T3, stat_incr::INT_T4]
    );
}
