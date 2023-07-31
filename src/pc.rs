use leptos::*;

pub mod craft;
mod description;
pub mod equip_slot;
mod followers;
mod inventory;
mod journal;
mod navbar;
mod overview;
mod scout;
pub mod session;
pub mod starting_equipment;

use equip_slot::EquipSlot;
pub use followers::Followers;
pub use inventory::{InvNavbar, Inventory, Vault};
pub use journal::Journal;
pub use overview::Overview;
pub use scout::PCScout;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter};

use self::description::gen_description;
use self::equip_slot::equip_items;
use self::starting_equipment as start_eq;
use crate::assets::ABILITY_MOD;
use crate::items::buffs::Buff;
use crate::items::recipes::{Recipe, RCP_SAGE};
use crate::items::Item;
use crate::rand::{rand_context, Rand};

pub const MAX_CAPACITY: u8 = 10;

#[derive(Serialize, Deserialize, Clone)]
pub struct PC {
    pub name: String,
    pub description: String,
    pub curr_hp: i32,
    pub wounds: i32,
    pub supply: u32,
    pub inventory: Vec<Item>,
    pub inv_count: u32,
    pub equipment: [Option<Item>; EquipSlot::COUNT],
    pub base_stats: [i32; PCStat::COUNT],
    pub recipes: [Option<Recipe>; 10],
    pub conditions: Vec<Buff>,
    pub days: u32,
    pub turns: u32,
}

impl PC {
    pub fn new(cx: Scope, name: String) -> Self {
        log::info!("creating new pc");
        rand_context(cx, |rand| {
            let mut inventory = start_eq::mage(rand);
            let inv_count = inventory.len() as u32;
            let equipment = equip_items(&mut inventory);
            let mut recipes = [0; 10].map(|_| None);
            recipes[0] = Some(RCP_SAGE.into());
            Self {
                name,
                description: gen_description(rand),
                curr_hp: 2,
                wounds: 0,
                // Somewhere between 10 silver and 20 silver
                supply: rand.range(10, 20) * 10,
                inventory,
                equipment,
                base_stats: gen_base_stats(rand),
                recipes,
                turns: 0,
                days: 0,
                conditions: Vec::new(),
                inv_count,
            }
        })
    }

    fn get_equipment(&self, equip_slot: EquipSlot) -> Option<&Item> {
        self.equipment[equip_slot.index()].as_ref()
    }

    /// Add the given item to the inventory and give it a unique id.
    fn add_inv_item(&mut self, item: &Item) {
        let mut item = item.clone();
        item.id = self.inv_count;
        self.inv_count += 1;
        self.inventory.push(item)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Display, EnumCount, EnumIter)]
#[allow(clippy::upper_case_acronyms)]
pub enum PCStat {
    HP,
    Speed,
    Sorcery,
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
    arr[PCStat::Sorcery.index()] = 0;
    arr[PCStat::STR.index()] = rand.pick(&ABILITY_MOD);
    arr[PCStat::DEX.index()] = rand.pick(&ABILITY_MOD);
    arr[PCStat::INT.index()] = rand.pick(&ABILITY_MOD);
    arr[PCStat::CHA.index()] = rand.pick(&ABILITY_MOD);
    arr
}
