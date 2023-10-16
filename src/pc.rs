use leptos::logging::log;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};

use self::description::gen_description;
use self::realm::Follower;
use crate::buffs::Buff;
use crate::items::{self, Item, SHOP_ADVENTURE_T1};
use crate::lobby::pc_basic::PCBasic;
use crate::pc::pc_class::PCClassRef;
use crate::rand::Rand;
use crate::utils::enum_array::{EnumArray, EnumRef};
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
pub mod realm;
pub mod scout;
pub mod session;
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
    pub base_stats: PCStatArray,
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

const PC_STAT_COUNT: usize = PCStat::COUNT;
pub type PCStatArray = EnumArray<PCStat, PC_STAT_COUNT>;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, strum::Display, EnumCount, EnumIter)]
#[allow(clippy::upper_case_acronyms)]
pub enum PCStat {
    Guard,
    Health,
    STR,
    DEX,
    INT,
    CHA,
}

impl EnumRef for PCStat {
    fn index(&self) -> usize {
        *self as usize
    }
}

fn gen_base_stats(rand: &mut Rand, class: PCClassRef) -> PCStatArray {
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
        PCClassRef::Fighter => &items::FIGHTER,
        PCClassRef::Rogue => &items::ROGUE,
        PCClassRef::Mage => &items::MAGE,
        PCClassRef::Cleric => &items::CLERIC,
    }
    .iter()
    .map(|x| (**x).into())
    .chain([(); 3].map(|_| (*rand.pick(&SHOP_ADVENTURE_T1)).into()))
    .collect()
}

pub mod attr {
    pub const MAX_INVENTORY: usize = 7;
}
