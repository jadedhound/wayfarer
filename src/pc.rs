use leptos::logging::log;
use serde::{Deserialize, Serialize};

use self::description::gen_description;
use self::pc_stat::{PCStat, StatArray};
use self::realm::Follower;
use crate::buffs::Buff;
use crate::items::simple::sundry;
use crate::items::{tome, weapons, Item};
use crate::lobby::pc_basic::PCBasic;
use crate::pc::pc_class::PCClassRef;
use crate::rand::Rand;
use crate::utils::index_map::IndexMap;
use crate::utils::turns::Turns;
use crate::utils::RwProvided;

pub mod class_view;
mod description;
pub mod inventory;
pub mod journal;
pub mod navbar;
pub mod overview;
pub mod pc_class;
pub mod pc_stat;
pub mod realm;
pub mod scout;
pub mod session;
pub mod shops;
mod update;

#[derive(Serialize, Deserialize, Clone)]
pub struct PC {
    pub name: String,
    pub class: (PCClassRef, u8),
    pub description: String,
    pub guard_dmg: i32,
    pub health_dmg: i32,
    pub funds: u32,
    pub inventory: IndexMap<Item>,
    pub quick_access: [Option<Item>; 3],
    pub base_stats: StatArray,
    pub buffs: IndexMap<Buff>,
    pub followers: IndexMap<Follower>,
    pub turns: Turns,
    pub prof: String,
}

impl From<PCBasic> for PC {
    fn from(value: PCBasic) -> Self {
        log!("Creating new PC");
        Rand::with(|rand| {
            Self {
                name: value.name,
                class: (value.class, 1),
                description: gen_description(rand),
                guard_dmg: 0,
                health_dmg: 0,
                // Somewhere between 10 silver and 20 silver
                funds: rand.range(10, 20) * 10,
                inventory: gen_inventory(rand, value.class),
                quick_access: [None, None, None],
                base_stats: gen_base_stats(rand, value.class),
                turns: Turns::default(),
                buffs: IndexMap::from(vec![Buff::from(*value.class.base_buffs[0])]),
                followers: IndexMap::default(),
                prof: value.class.prof.into(),
            }
        })
    }
}

impl RwProvided for PC {
    type Item = Self;
}

fn gen_base_stats(rand: &mut Rand, class: PCClassRef) -> StatArray {
    // -2: 10%, -1: 20%, 0: 15%, +1: 25%, +2: 20%, +3: 10%
    const ABILITY_MOD: [i32; 20] = [
        -2, -2, -1, -1, -1, -1, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3,
    ];
    let mut ability_arr = [0; 4].map(|_| rand.pick(&ABILITY_MOD));
    // Sort by smallest to largest.
    ability_arr.sort_unstable();
    // Adjust to proper index. [ STR, DEX, INT, CHA ].
    let stat_priority = match class {
        PCClassRef::Fighter => [1, 2, 4, 3],
        PCClassRef::Rogue => [4, 1, 3, 2],
        PCClassRef::Mage => [4, 3, 1, 2],
        PCClassRef::Cleric => [2, 4, 3, 1],
    }
    // Index 3 is largest abi, 0 is smallest.
    .map(|i| ability_arr[4 - i]);
    vec![
        (PCStat::Guard, 6),
        (PCStat::Health, 2),
        (PCStat::STR, stat_priority[0]),
        (PCStat::DEX, stat_priority[1]),
        (PCStat::INT, stat_priority[2]),
        (PCStat::CHA, stat_priority[3]),
    ]
    .into()
}

fn gen_inventory(rand: &mut Rand, class: PCClassRef) -> IndexMap<Item> {
    match class {
        PCClassRef::Fighter => vec![&weapons::HAMMER, &weapons::SHIELD],
        PCClassRef::Rogue => vec![&weapons::SWORD, &weapons::DAGGER],
        PCClassRef::Mage => vec![rand.pick(&tome::spell::ALL)],
        PCClassRef::Cleric => vec![rand.pick(&tome::prayer::ALL)],
    }
    .into_iter()
    .map(|x| (*x).into())
    .chain([(); 3].map(|_| (*rand.pick(&sundry::ALL)).into()))
    .collect()
}

pub mod attr {
    pub const MAX_INVENTORY: usize = 7;
}
