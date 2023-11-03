use leptos::logging::log;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};

use self::class::level::ClassExp;
use crate::buffs::Buff;
use crate::items::{self, Item, ItemProp};
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
pub mod edit_item;
pub mod inventory;
pub mod journal;
mod navbar;
pub mod realm;
pub mod scout;
pub mod session;
mod update;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PC {
    pub name: String,
    pub class: (PCClassRef, ClassExp),
    pub prof: String,
    pub abi_scores: AbiScores,
    pub guard_dmg: u32,
    pub health_dmg: u32,
    pub wealth: u32,
    pub inventory: IndexMap<Item>,
    pub recently_removed: FixedVec<Item>,
    pub quick_access: FixedVec<usize>,
    pub buffs: IndexMap<Buff>,
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
            quick_access: FixedVec::new(2),
            buffs: IndexMap::from(vec![Buff::from(*value.class.base_buffs[0])]),
            recently_removed: FixedVec::new(10),
            ..Default::default()
        }
    }
}

impl PC {
    /// Removes an item from inventory, adding it to `pc.recently_removed`
    /// removing it from `pc.quick_access`.
    fn inventory_remove(&mut self, id: usize) {
        self.quick_access.remove_where(|x| *x == id);
        if let Some(mut item) = self.inventory.remove(id) {
            let unique = !self.recently_removed.iter().any(|removed| removed == &item);
            if unique {
                if let Some(count) = item.find_mut_counter() {
                    count.curr = count.max
                }
                self.recently_removed.push(item)
            }
        }
    }

    /// Consume an item (or item count) and apply its effects (if any).
    fn use_item(&mut self, id: usize) {
        let item = self.inventory.get_mut(id).unwrap();
        for prop in item.props.iter() {
            if let ItemProp::Buff(buff) = prop {
                self.buffs.add(buff.clone())
            }
        }
        if let Some(count) = item.find_mut_counter() {
            count.curr -= 1;
            if count.is_empty() {
                self.inventory_remove(id);
            }
        } else {
            self.inventory_remove(id);
        }
    }
}

impl RwUtils for PC {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, strum::Display, EnumCount, EnumIter)]
#[allow(clippy::upper_case_acronyms)]
pub enum Ability {
    Guard,
    Health,
    #[strum(serialize = "Max Inventory")]
    MaxInventory,
    #[strum(serialize = "Quick Access")]
    QuickAccess,
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
            (Ability::MaxInventory, 10),
            (Ability::QuickAccess, 2),
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

fn gen_inventory(class: PCClassRef) -> IndexMap<Item> {
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
        .collect()
}
