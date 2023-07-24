use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::enhancement::Enhancement;
use crate::utils::LazyHash;

pub static GEMS: LazyHash<Gem> = Lazy::new(|| HashMap::new());

pub static RUNES: LazyHash<Rune> = Lazy::new(|| HashMap::new());

#[derive(Serialize, Deserialize, Clone)]
pub struct Gem {
    pub name: String,
    pub prefix: String,
    pub effect: Vec<Enhancement>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Rune {
    pub name: String,
    pub suffix: String,
    pub effect: Vec<Enhancement>,
}
