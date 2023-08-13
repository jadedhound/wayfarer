use leptos::*;

pub mod craft;
mod description;
pub mod followers;
pub mod inventory;
pub mod journal;
pub mod navbar;
pub mod overview;
pub mod scout;
pub mod session;
pub mod starting_equipment;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter};

use self::description::gen_description;
use self::starting_equipment as start_eq;
use crate::assets::ABILITY_MOD;
use crate::items::buffs::Buff;
use crate::items::recipes::{Recipe, RCP_SAGE};
use crate::items::Item;
use crate::rand::{rand_context, Rand};
use crate::utils::index_map::IndexMap;

pub const MAX_CAPACITY: usize = 10;

#[derive(Serialize, Deserialize, Clone)]
pub struct PC {
    pub name: String,
    pub description: String,
    pub curr_hp: i32,
    pub wounds: i32,
    pub supply: u32,
    pub inventory: IndexMap<Item>,
    pub quick_access: [Option<Item>; 3],
    pub base_stats: [i32; PCStat::COUNT],
    pub recipes: [Option<Recipe>; 5],
    pub buffs: IndexMap<Buff>,
    pub turns: u64,
}

impl PC {
    pub fn new(cx: Scope, name: String) -> Self {
        log::info!("creating new pc");
        rand_context(cx, |rand| {
            let mut recipes = [0; 5].map(|_| None);
            recipes[0] = Some(RCP_SAGE.into());
            Self {
                name,
                description: gen_description(rand),
                curr_hp: rand.range(3, 6) as i32,
                wounds: 0,
                // Somewhere between 10 silver and 20 silver
                supply: rand.range(10, 20) * 10,
                inventory: IndexMap::new(start_eq::mage(rand)),
                quick_access: [None, None, None],
                base_stats: gen_base_stats(rand),
                recipes,
                turns: 0,
                buffs: IndexMap::new(Vec::new()),
            }
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Display, EnumCount, EnumIter)]
#[allow(clippy::upper_case_acronyms)]
pub enum PCStat {
    HP,
    Speed,
    STR,
    DEX,
    INT,
    CHA,
}

impl PCStat {
    pub const fn index(&self) -> usize {
        *self as usize
    }
}

fn gen_base_stats(rand: &mut Rand) -> [i32; PCStat::COUNT] {
    let mut arr = [0; PCStat::COUNT];
    arr[PCStat::HP.index()] = 1;
    arr[PCStat::Speed.index()] = 30;
    arr[PCStat::STR.index()] = rand.pick(&ABILITY_MOD);
    arr[PCStat::DEX.index()] = rand.pick(&ABILITY_MOD);
    arr[PCStat::INT.index()] = rand.pick(&ABILITY_MOD);
    arr[PCStat::CHA.index()] = rand.pick(&ABILITY_MOD);
    arr
}
