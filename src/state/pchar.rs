use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct PChar {
    pub name: String,
}

impl PChar {
    pub fn new() -> Self {
        Self {
            name: "Test".to_string(),
        }
    }
}
