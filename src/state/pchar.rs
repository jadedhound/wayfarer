use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct PChar {
    pub id: u64,
    pub name: String,
}

impl PChar {
    pub fn new(name: String) -> Self {
        Self {
            id: js_sys::Date::now() as u64,
            name,
        }
    }
}
