use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Serialize, Deserialize, Clone)]
pub struct Reagent {
    r_type: ReagentType,
    quality: u8,
}

impl Reagent {
    pub fn quality_str(&self) -> &'static str {
        match self.quality {
            3 => "Mythical",
            2 => "Rare",
            1 => "Uncommon",
            0 => "Common",
            _ => "Error",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, AsRefStr)]
pub enum ReagentType {
    Steel,
    Leather,
    Cloth,
    Fungi,
}
