use std::collections::HashMap;

use serde::Deserialize;

use crate::errors::*;
use crate::utils::*;

pub mod details;
pub mod list;

type Features = HashMap<String, String>;
type AllClasses = HashMap<String, PClass>;

#[derive(Deserialize, Clone)]
struct PClass {
    desc: String,
    adv_table: [String; 4],
    basics: PCBasics,
    equipment: Vec<String>,
    core: Option<Features>,
    archetypes: HashMap<String, Archetype>,
}

#[derive(Deserialize, Clone)]
struct Archetype {
    prof: String,
    features: Features,
}

#[derive(Deserialize, Clone)]
struct PCBasics {
    starting_hp: u8,
    level_hp: u8,
    armour_prof: String,
    weap_prof: String,
}
