use leptos::logging::log;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};

use self::class::level::ClassExp;
use crate::items::{self, Item};
use crate::lobby::pc_basic::PCBasic;
use crate::pc::class::PCClassRef;
use crate::rand::Rand;
use crate::utils::enum_array::{EnumArray, EnumRef};
use crate::utils::fixed_vec::FixedVec;
use crate::utils::inventory::Inventory;
use crate::utils::rw_utils::RwUtils;
use crate::utils::turns::Turns;

pub mod class;
pub mod edit_item;
pub mod inventory;
pub mod journal;
pub mod main;
mod navbar;
pub mod realm;
pub mod scout;
pub mod session;
mod update;

const MAX_INVENTORY: usize = 10;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PC {
    pub name: String,
    pub class: (PCClassRef, ClassExp),
    pub prof: String,
    pub abi_scores: AbiScores,
    pub guard_dmg: u32,
    pub health_dmg: u32,
    pub wealth: u32,
    pub inventory: Inventory,
    pub recently_removed: FixedVec<Item>,
    pub turns: Turns,
    pub open_notes: Vec<usize>,
    pub fatigue: i32,
}

impl From<PCBasic> for PC {
    fn from(value: PCBasic) -> Self {
        log!("Creating new PC");
        Self {
            name: value.name,
            class: (value.class, ClassExp::default()),
            abi_scores: AbiScores::from(value.class),
            prof: value.class.prof.into(),
            wealth: 15 * 10,
            inventory: gen_inventory(value.class),
            recently_removed: FixedVec::new(10),
            ..Default::default()
        }
    }
}

impl RwUtils for PC {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, strum::Display, EnumCount, EnumIter)]
#[allow(clippy::upper_case_acronyms)]
pub enum Ability {
    Guard,
    Health,
    STR,
    DEX,
    INT,
    CHA,
}

impl EnumRef for Ability {
    fn index(&self) -> usize {
        *self as usize
    }
}

pub type AbiScores = EnumArray<Ability, { Ability::COUNT }>;

impl From<PCClassRef> for AbiScores {
    fn from(value: PCClassRef) -> Self {
        // -2: 10%, -1: 20%, 0: 15%, +1: 25%, +2: 20%, +3: 10%
        const ABILITY_MOD: [i32; 20] = [
            -2, -2, -1, -1, -1, -1, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3,
        ];
        let mut rand_abi = Rand::with(|rand| [0; 4].map(|_| rand.pick(&ABILITY_MOD)));
        // Sort by smallest to largest.
        rand_abi.sort_unstable();
        // Adjust to proper index. [ STR, DEX, INT, CHA ].
        let stat_priority = match value {
            PCClassRef::Fighter => [1, 2, 4, 3],
            PCClassRef::Rogue => [4, 1, 3, 2],
            PCClassRef::Mage => [4, 3, 1, 2],
            PCClassRef::Cleric => [2, 4, 3, 1],
        }
        // Index 3 is largest abi, 0 is smallest.
        .map(|i| rand_abi[4 - i]);
        vec![
            (Ability::Guard, 6),
            (Ability::Health, 2),
            (Ability::STR, stat_priority[0]),
            (Ability::DEX, stat_priority[1]),
            (Ability::INT, stat_priority[2]),
            (Ability::CHA, stat_priority[3]),
        ]
        .into()
    }
}

impl PartialEq for AbiScores {
    fn eq(&self, other: &Self) -> bool {
        let is_not_same = self
            .iter_enum()
            .any(|(stat, value)| other.get(stat) != value);
        !is_not_same
    }
}

fn gen_inventory(class: PCClassRef) -> Inventory {
    let adventuring_gear: Vec<_> = Rand::with(|rand| {
        vec![(); class.adventuring_gear]
            .into_iter()
            .map(|_| Item::from(*rand.pick(&items::adventure::ITEMS)))
            .collect()
    });
    class
        .starter_gear
        .iter()
        .chain([&&items::adventure::TORCH])
        .map(|item_ref| Item::from(**item_ref))
        .chain(adventuring_gear)
        .collect::<Vec<_>>()
        .into()
}
