use leptos::logging::log;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};

use self::class::level::ClassExp;
use self::description::gen_description;
use self::realm::Follower;
use crate::buffs::Buff;
use crate::items::{self, Item};
use crate::lobby::pc_basic::PCBasic;
use crate::pc::class::PCClassRef;
use crate::rand::Rand;
use crate::utils::enum_array::{EnumArray, EnumRef};
use crate::utils::fixed_vec::FixedVec;
use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;
use crate::utils::turns::Turns;

pub mod class;
pub mod combat;
mod description;
pub mod inventory;
pub mod journal;
pub mod navbar;
pub mod realm;
pub mod scout;
pub mod session;
mod update;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PC {
    pub name: String,
    // Class and EXP.
    pub class: (PCClassRef, ClassExp),
    pub description: String,
    pub guard_dmg: i32,
    pub health_dmg: i32,
    pub wealth: u32,
    pub inventory: IndexMap<Item>,
    pub quick_access: FixedVec<usize>,
    pub base_stats: PCStatArray,
    pub buffs: IndexMap<Buff>,
    pub followers: IndexMap<Follower>,
    pub follower_cooldown: Turns,
    pub turns: Turns,
    pub prof: String,
}

impl From<PCBasic> for PC {
    fn from(value: PCBasic) -> Self {
        log!("Creating new PC");
        Rand::with(|rand| {
            Self {
                name: value.name,
                class: (value.class, ClassExp::default()),
                description: gen_description(rand),
                // 15 SP to start.
                wealth: 150,
                inventory: gen_inventory(rand, value.class),
                quick_access: FixedVec::new(2),
                base_stats: gen_base_stats(rand, value.class),
                buffs: IndexMap::from(vec![Buff::from(*value.class.base_buffs[0])]),
                prof: value.class.prof.into(),
                ..Default::default()
            }
        })
    }
}

impl RwUtils for PC {
    type Item = Self;
}

const PC_STAT_COUNT: usize = PCStat::COUNT;
pub type PCStatArray = EnumArray<PCStat, PC_STAT_COUNT>;

impl PartialEq for PCStatArray {
    fn eq(&self, other: &Self) -> bool {
        let is_not_same = self
            .iter_enum()
            .any(|(stat, value)| other.get(stat) != value);
        !is_not_same
    }
}

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
        (PCStat::Guard, attr::GUARD),
        (PCStat::Health, attr::HEALTH),
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
    .chain([(); 3].map(|_| (*rand.pick(&items::adventure::ITEMS)).into()))
    .collect()
}

mod attr {
    pub const INV_CAPACITY: usize = 10;
    pub const HEALTH: i32 = 2;
    pub const GUARD: i32 = 6;
}
