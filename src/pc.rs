pub mod craft;
mod description;
pub mod followers;
pub mod inventory;
pub mod journal;
pub mod navbar;
pub mod overview;
pub mod pc_stat;
pub mod scout;
pub mod session;
pub mod starting_equipment;
mod update;

use serde::{Deserialize, Serialize};

use self::description::gen_description;
use self::followers::Follower;
use self::pc_stat::{PCStat, StatArray};
use self::starting_equipment as start_eq;
use crate::items::buffs::Buff;
use crate::items::recipes::{self, Recipe};
use crate::items::Item;
use crate::pc::pc_stat::StatArrBuilder;
use crate::rand::Rand;
use crate::utils::index_map::IndexMap;
use crate::utils::time::Turns;
use crate::utils::RwProvided;

#[derive(Serialize, Deserialize, Clone)]
pub struct PC {
    pub name: String,
    pub description: String,
    pub stamina_dmg: i32,
    pub health_dmg: i32,
    pub funds: u32,
    pub inventory: IndexMap<Item>,
    pub quick_access: [Option<Item>; 3],
    pub base_stats: StatArray,
    pub recipes: Vec<Recipe>,
    pub buffs: IndexMap<Buff>,
    pub followers: IndexMap<Follower>,
    pub turns: Turns,
}

impl PC {
    pub fn new(name: String) -> Self {
        log::info!("creating new pc");
        Rand::with(|rand| {
            Self {
                name,
                description: gen_description(rand),
                stamina_dmg: 0,
                health_dmg: 0,
                // Somewhere between 10 silver and 20 silver
                funds: rand.range(10, 20) * 10,
                inventory: IndexMap::from(start_eq::mage(rand)),
                quick_access: [None, None, None],
                base_stats: gen_base_stats(rand),
                recipes: vec![recipes::CUNNING.into()],
                turns: Turns::new(0),
                buffs: IndexMap::default(),
                followers: IndexMap::default(),
            }
        })
    }
}

impl RwProvided for PC {
    type Item = Self;
}

fn gen_base_stats(rand: &mut Rand) -> StatArray {
    /// -2: 10%, -1: 20%, 0: 15%, +1: 25%, +2: 20%, +3: 10%
    const ABILITY_MOD: [i32; 20] = [
        -2, -2, -1, -1, -1, -1, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3,
    ];
    let mut ability_arr = [0; 4].map(|_| rand.pick(&ABILITY_MOD));
    // Sort by smallest to largest.
    ability_arr.sort_unstable();
    let stat_priority = [3, 1, 0, 2].map(|i| ability_arr[i]);
    StatArrBuilder::new()
        .stam(6)
        .health(2)
        .speed(30)
        .recipes(3)
        .inventory(6)
        .str(stat_priority[0])
        .dex(stat_priority[1])
        .int(stat_priority[2])
        .cha(stat_priority[3])
        .build()
}
