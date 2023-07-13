use leptos::*;
mod basics;
mod crafting;
mod generators;
mod inventory;
mod navbar;
mod scout;

pub use basics::Basics;
pub use crafting::Crafting;
use generators::*;
pub use inventory::{InvNavbar, Inventory, Vault};
pub use scout::PCScout;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator};

use crate::assets::ABILITY_MOD;
use crate::items::enhancement::{self as enh, Enhancement, Feature};
use crate::items::{Armour, GearType, Held, Item};
use crate::rand::{rand_context, Rand};
use crate::utils::{read_context, EnumIndex, EnumMap};

/// Gives the damage die to be used for a given `power`.
/// Defaults to maximum die size if power is too large (or small).
pub fn damage_die(power: usize) -> String {
    const DAMAGE_DICE: [&str; 10] = [
        "1", "1d4", "1d6", "1d8", "1d10", "1d12", "2d6", "2d8", "2d10", "2d12",
    ];
    DAMAGE_DICE
        .get(power)
        .unwrap_or(&DAMAGE_DICE[9])
        .to_string()
}

#[derive(Serialize, Deserialize, Clone)]
struct PC {
    pub name: String,
    pub description: String,
    pub curr_hp: i32,
    pub inventory: Vec<Item>,
    pub equipment: EnumMap<Option<usize>>,
    pub base_stats: EnumMap<i32>,
}

impl PC {
    pub fn new(cx: Scope, name: String) -> Self {
        rand_context(cx, |rand| {
            let inventory = gen_warrior_inv(rand);
            let equipment = equip_items(&inventory);
            Self {
                name,
                description: "TODO: Generate random description.".into(),
                curr_hp: 5,
                inventory,
                equipment,
                base_stats: gen_base_stats(rand),
            }
        })
    }

    /// Fetchs the item in the slot given.
    pub fn get_equip(&self, slot: EquipSlot) -> Option<Item> {
        let i = (*self.equipment.get(slot))?;
        self.inventory.get(i).cloned()
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
    Damage,
    STR,
    DEX,
    INT,
    CHA,
}

impl EnumIndex for PCStat {
    fn index(&self) -> usize {
        *self as usize
    }
}

fn gen_base_stats(rand: &mut Rand) -> EnumMap<i32> {
    let mut arr = EnumMap::new::<PCStat>(0);
    *arr.get_mut(PCStat::HP) = 5;
    *arr.get_mut(PCStat::Speed) = 30;
    *arr.get_mut(PCStat::Sorcery) = 0;
    *arr.get_mut(PCStat::Damage) = 0;
    *arr.get_mut(PCStat::STR) = rand.pick(&ABILITY_MOD);
    *arr.get_mut(PCStat::DEX) = rand.pick(&ABILITY_MOD);
    *arr.get_mut(PCStat::INT) = rand.pick(&ABILITY_MOD);
    *arr.get_mut(PCStat::CHA) = rand.pick(&ABILITY_MOD);
    arr
}

// -----------------------------------
// PC SESSION
// -----------------------------------

#[derive(Serialize, Deserialize, Clone)]
pub struct PCSession {
    stats: EnumMap<i32>,
    features: Vec<Feature>,
}

impl PCSession {
    /// Calculates modified stats from the PC's inventory.
    /// Relies on `PC` to have been provided already.
    pub fn new(cx: Scope) -> Self {
        read_context::<PC>(cx).with_untracked(|pc| {
            let stats = pc.base_stats.clone();
            let features = Vec::new();
            let mut result = Self { stats, features };
            EquipSlot::iter()
                .filter_map(|slot| {
                    let i = (*pc.equipment.get(slot))?;
                    let item = pc.inventory.get(i)?;
                    Some((slot, item))
                })
                .for_each(|(slot, item)| add_item(slot, item, &mut |enh| result.add_enh(enh)));
            result
        })
    }

    fn add_enh(&mut self, enh: Enhancement) {
        match enh {
            Enhancement::StatInc(si) => {
                *self.stats.get_mut(si.stat) += si.add;
            }
            Enhancement::Feature(f) => self.features.push(f),
        }
    }
}

fn add_item<F>(slot: EquipSlot, item: &Item, f: &mut F)
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
                        PCStat::Damage => Some(enh::stat::DAMAGE_1),
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

impl EnumIndex for EquipSlot {
    fn index(&self) -> usize {
        *self as usize
    }
}

fn equip_items(inv: &[Item]) -> EnumMap<Option<usize>> {
    let mut arr = EnumMap::new::<EquipSlot>(None);
    inv.iter().enumerate().for_each(|(i, item)| match item {
        Item::Held(held) => match held.base {
            Held::Shield => *arr.get_mut(EquipSlot::OffHand) = Some(i),
            _ => *arr.get_mut(EquipSlot::MainHand) = Some(i),
        },
        Item::Head(_) => *arr.get_mut(EquipSlot::Head) = Some(i),
        Item::Armour(armour) => match armour.base {
            Armour::Leggings | Armour::Chausses | Armour::Greaves => {
                *arr.get_mut(EquipSlot::Legs) = Some(i)
            }
            _ => *arr.get_mut(EquipSlot::Body) = Some(i),
        },
        _ => (),
    });
    arr
}
