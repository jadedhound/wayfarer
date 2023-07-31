use serde::{Deserialize, Serialize};

use super::potions::{POT_AWKND_SHRUB, POT_SAGE};
use super::reagents::SAGEROOT;
use super::{Item, ItemRef};

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub ingredients: [Option<(Item, u8)>; 2],
    pub success: Item,
    pub failure: Item,
}

impl From<RecipeRef> for Recipe {
    fn from(value: RecipeRef) -> Self {
        Self {
            ingredients: value
                .ingredients
                .map(|x| x.map(|(item, y)| ((*item).into(), y))),
            success: (*value.success).into(),
            failure: (*value.failure).into(),
        }
    }
}

pub struct RecipeRef {
    pub ingredients: [Option<(&'static ItemRef, u8)>; 2],
    success: &'static ItemRef,
    failure: &'static ItemRef,
}

const fn rcp_1(
    item: &'static ItemRef,
    ammount: u8,
    success: &'static ItemRef,
    failure: &'static ItemRef,
) -> RecipeRef {
    RecipeRef {
        ingredients: [Some((item, ammount)), None],
        success,
        failure,
    }
}

pub const RCP_SAGE: RecipeRef = rcp_1(&SAGEROOT, 2, &POT_SAGE, &POT_AWKND_SHRUB);
