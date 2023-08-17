use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

use crate::{
    items::{reagents::flora, Item, ItemRef},
    rand::Rand,
};

#[derive(Serialize, Deserialize, Clone, Copy, AsRefStr, EnumIter)]
pub enum ForageTable {
    #[strum(to_string = "forest common")]
    ForestT1,
}

impl ForageTable {
    pub fn to_item(self) -> Item {
        let table = match self {
            Self::ForestT1 => &tables::FOREST_T1,
        };
        (*Rand::with(|rand| rand.pick(table))).into()
    }
}

#[rustfmt::skip]
mod tables {
    use super::*;
    
    pub(super) const FOREST_T1: [&ItemRef; 6] = [
        &flora::SAGEROOT, &flora::SAGEROOT, &flora::SAGEROOT, &flora::SAGEROOT, 
        &flora::FUNGI, &flora::FUNGI
    ];
}
