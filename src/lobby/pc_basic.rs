use serde::{Deserialize, Serialize};

use crate::pc::pc_class::PCClassRef;

#[derive(Serialize, Deserialize, Clone)]
pub struct PCBasic {
    pub name: String,
    pub class: PCClassRef,
}

impl Default for PCBasic {
    fn default() -> Self {
        Self {
            name: String::new(),
            class: PCClassRef::Fighter,
        }
    }
}
