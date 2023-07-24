use leptos::*;
mod crafting;
mod description;
mod followers;
mod generators;
mod inventory;
mod journal;
mod navbar;
mod overview;
mod scout;

pub use crafting::Crafting;
pub use followers::Followers;
use generators::*;
pub use inventory::{InvNavbar, Inventory, Vault};
pub use journal::Journal;
pub use overview::Overview;
pub use scout::PCScout;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator};

use self::description::gen_description;
use crate::assets::ABILITY_MOD;
use crate::items::enhancement::{self as enh, Enhancement, Feature};
use crate::items::{Armour, GearType, Held, Item};
use crate::rand::{rand_context, Rand};
use crate::utils::{read_context, VecStrOps};

pub const MAX_CAPACITY: u8 = 10;

#[derive(Serialize, Deserialize, Clone)]
pub struct PC {
    pub name: String,
    pub description: String,
    pub curr_hp: i32,
    pub wounds: i32,
    pub supply: u32,
    pub inventory: Vec<Item>,
    pub equipment: [Option<Item>; EquipSlot::COUNT],
    pub base_stats: [i32; PCStat::COUNT],
    pub days: u32,
}

impl PC {
    pub fn new(cx: Scope, name: String) -> Self {
        rand_context(cx, |rand| {
            let mut inventory = gen_warrior_inv(rand);
            let equipment = equip_items(&mut inventory);
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
                days: 0,
            }
        })
    }
}

// -----------------------------------
// PC STATS
// -----------------------------------

#[derive(Serialize, Deserialize, Clone, Copy, Display, EnumCount)]
#[allow(clippy::upper_case_acronyms)]
pub enum PCStat {
    HP,
    Speed,
    Sorcery,
    Might,
    STR,
    DEX,
    INT,
    CHA,
}

impl PCStat {
    fn index(&self) -> usize {
        *self as usize
    }
}

fn gen_base_stats(rand: &mut Rand) -> [i32; PCStat::COUNT] {
    let mut arr = [0; PCStat::COUNT];
    arr[PCStat::HP.index()] = 5;
    arr[PCStat::Speed.index()] = 30;
    arr[PCStat::Sorcery.index()] = 0;
    arr[PCStat::Might.index()] = 0;
    arr[PCStat::STR.index()] = rand.pick(&ABILITY_MOD);
    arr[PCStat::DEX.index()] = rand.pick(&ABILITY_MOD);
    arr[PCStat::INT.index()] = rand.pick(&ABILITY_MOD);
    arr[PCStat::CHA.index()] = rand.pick(&ABILITY_MOD);
    arr
}

// -----------------------------------
// PC SESSION
// -----------------------------------

#[derive(Serialize, Deserialize, Clone)]
pub struct PCSession {
    stats: [i32; PCStat::COUNT],
    features: Vec<Feature>,
}

impl PCSession {
    /// Calculates modified stats from the PC's inventory.
    /// Relies on `PC` to have been provided already.
    pub fn new(cx: Scope) -> Self {
        read_context::<PC>(cx).with_untracked(|pc| {
            let stats = pc.base_stats;
            let features = Vec::new();
            let mut result = Self { stats, features };
            EquipSlot::iter()
                .filter_map(|slot| {
                    let item = pc.equipment.as_ref()[slot.index()].clone()?;
                    Some((slot, item))
                })
                .for_each(|(slot, item)| {
                    filter_appliable_enh(slot, &item, &mut |enh| result.add_enh(enh))
                });
            result
        })
    }

    pub fn add_enh(&mut self, enh: Enhancement) {
        match enh {
            Enhancement::StatInc(si) => {
                self.stats[si.stat.index()] += si.add;
            }
            Enhancement::Feature(f) => self.features.push(f),
        }
    }

    pub fn remove_enh(&mut self, enh: Enhancement) {
        match enh {
            Enhancement::StatInc(si) => {
                self.stats[si.stat.index()] -= si.add;
            }
            Enhancement::Feature(f) => {
                if let Some(pos) = self.features.iter().position(|p| p.name == f.name) {
                    self.features.remove(pos);
                }
            }
        }
    }
}

pub fn filter_appliable_enh<F>(slot: EquipSlot, item: &Item, f: &mut F)
where
    F: FnMut(Enhancement),
{
    match slot {
        EquipSlot::MainHand => {
            let item = item.as_held().unwrap();
            // Add gem and base stats/feats as normal
            for e in item.base.enhancements().into_iter() {
                f(e)
            }
            if let Some(gem) = item.gem {
                for e in gem.effect.iter().cloned() {
                    f(e)
                }
            }
            // Only apply feats and not additional stats
            if let Some(rune) = item.rune {
                rune.effect
                    .iter()
                    .filter(|e| e.is_feat())
                    .cloned()
                    .for_each(f)
            }
        }
        EquipSlot::OffHand => {
            let item = item.as_held().unwrap();
            // Only add base stats not associated with damage
            let only_stats = item.base.enhancements().into_iter().filter_map(|e| {
                let orig = e.clone();
                match e {
                    Enhancement::StatInc(si) => match si.stat {
                        // Dual wielding should max increase damage by 1 regardless of weapon
                        PCStat::Might => Some(enh::stat::DAMAGE_1),
                        _ => Some(orig),
                    },
                    _ => None,
                }
            });
            for e in only_stats {
                f(e)
            }
            // Add gem like normal
            if let Some(gem) = item.gem {
                for e in gem.effect.iter().cloned() {
                    f(e)
                }
            }
            // Only apply stats and not feats
            if let Some(rune) = item.rune {
                rune.effect
                    .iter()
                    .filter(|e| e.is_stat())
                    .cloned()
                    .for_each(f)
            }
        }
        _ => match item {
            Item::Head(_) => todo!(),
            Item::Armour(item) => {
                // Only add gem and base stats/feats as normal
                for e in item.base.enhancements().into_iter() {
                    f(e)
                }
                if let Some(stat) = item.gem.as_ref() {
                    for e in stat.effect.iter().cloned() {
                        f(e)
                    }
                }
                // Only add rune stats and not feats
                if let Some(rune) = item.rune.as_ref() {
                    rune.effect
                        .iter()
                        .filter(|e| !e.is_feat())
                        .cloned()
                        .for_each(f)
                }
            }
            _ => (),
        },
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Display, EnumCount, EnumIter)]
pub enum EquipSlot {
    Head,
    #[strum(serialize = "Main Hand")]
    MainHand,
    #[strum(serialize = "Off Hand")]
    OffHand,
    Body,
    Legs,
}

impl EquipSlot {
    fn index(&self) -> usize {
        *self as usize
    }
}

/// Takes and equips items to an appropriate slot.
fn equip_items(inv: &mut Vec<Item>) -> [Option<Item>; EquipSlot::COUNT] {
    let mut to_delete = Vec::new();
    let mut arr = [None, None, None, None, None];
    let mut assign = |slot: EquipSlot, i, item: &Item| {
        arr[slot.index()] = Some(item.clone());
        to_delete.push(i);
    };
    // Assign items by cloning
    inv.iter().enumerate().for_each(|(i, item)| match item {
        Item::Held(held) => match held.base {
            Held::Shield => assign(EquipSlot::OffHand, i, item),
            _ => assign(EquipSlot::MainHand, i, item),
        },
        Item::Head(_) => assign(EquipSlot::Head, i, item),
        Item::Armour(armour) => match armour.base {
            Armour::Leggings | Armour::Chausses | Armour::Greaves => {
                assign(EquipSlot::Legs, i, item)
            }
            _ => assign(EquipSlot::Body, i, item),
        },
        _ => (),
    });
    // Remove from inventory
    to_delete.sort_unstable();
    for i in to_delete.into_iter().rev() {
        inv.remove(i);
    }
    arr
}

fn format_funds(f: u32) -> String {
    let above_zero = |val, s| {
        if val > 0 {
            Some(format!("{val}{s}"))
        } else {
            None
        }
    };
    let mut f = f;
    let c = f % 10;
    f /= 10;
    let s = f % 100;
    f /= 100;
    vec![
        above_zero(f, "gp"),
        above_zero(s, "sp"),
        above_zero(c, "cp"),
    ]
    .flat_concat(" ")
    .unwrap_or("0cp".to_string())
}
