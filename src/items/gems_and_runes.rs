use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::enhancement::Enhancement;
use crate::utils::{cmphash_from_hash, CharHash, LazyHash};

pub static GEMS: LazyHash<Gem> = Lazy::new(|| HashMap::new());

pub static GEMS_CMPHASH: LazyHash<CharHash> = Lazy::new(|| cmphash_from_hash(&GEMS));

pub static RUNES: LazyHash<Rune> = Lazy::new(|| HashMap::new());

pub static RUNES_CMPHASH: LazyHash<CharHash> = Lazy::new(|| cmphash_from_hash(&RUNES));

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
