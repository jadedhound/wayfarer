use std::cmp::Ordering;
use std::collections::HashMap;

use leptos::*;
use serde::{Deserialize, Serialize};

use super::index_map::IndexMap;
use crate::icons;
use crate::items::Item;
use crate::pc::PC;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Inventory {
    items: IndexMap<Item>,
    sorted: Vec<usize>,
    weight: HashMap<usize, SlotRange>,
    max_size: usize,
}

impl Inventory {
    /// Add an item.
    pub fn add(&mut self, item: Item) {
        let name = item.name.clone();
        let id = self.items.add(item);
        self.add_sorted(id, name);
        self.calc_weight();
    }
    /// Remove an item.
    pub fn remove(&mut self, id: usize) -> Option<Item> {
        if let Some(i) = self.sorted.iter().position(|&i| i == id) {
            self.sorted.remove(i);
        }
        self.calc_weight();
        self.items.remove(id)
    }
    /// Consume an item (or item count) and apply its effects (if any).
    pub fn use_item(&mut self, id: usize) -> Option<Item> {
        let item = self.items.get_mut(id).unwrap();
        if let Some(count) = item.find_mut_counter() {
            count.curr -= 1;
            if count.is_empty() {
                self.remove(id)
            } else {
                None
            }
        } else {
            self.remove(id)
        }
    }
    /// Get an item for a given `id`.
    pub fn get(&self, id: usize) -> Option<&Item> {
        self.items.get(id)
    }
    /// Mutable version of `get`.
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Item> {
        self.items.get_mut(id)
    }
    /// List of items sorted by name.
    pub fn values(&self) -> impl Iterator<Item = &Item> + '_ {
        self.sorted.iter().flat_map(|i| self.items.get(*i))
    }
    /// List of items and their ids sorted by name.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &Item)> + '_ {
        self.sorted
            .iter()
            .flat_map(|i| Some((*i, self.items.get(*i)?)))
    }
    /// List of item ids.
    pub fn keys(&self) -> impl Iterator<Item = usize> + '_ {
        self.sorted.iter().copied()
    }
    /// Length.
    pub fn len(&self) -> usize {
        self.sorted.len()
    }
    /// The amount of vacant item slots left or `None` if encumbered.
    pub fn vacancy(&self) -> Option<usize> {
        Some(self.max_size.saturating_sub(self.size()?))
    }
    /// The current size of the inventory, which will not exceed `max_size`
    /// regardless of how many items there are actually in the inventory.
    pub fn size(&self) -> Option<usize> {
        self.sorted
            .last()
            .and_then(|i| self.weight.get(i))
            .copied()
            .unwrap_or_default()
            .largest()
    }
    /// The current maximum number of item slots.
    pub fn max_size(&self) -> usize {
        self.max_size
    }
    /// Changes the maximum number of item slots and recalculates slot ranges.
    pub fn resize(&mut self, max_size: usize) {
        self.max_size = max_size;
        self.calc_weight();
    }
    /// Gets the `SlotRange` for a given `id`
    pub fn get_slot(&self, id: usize) -> SlotRange {
        self.weight.get(&id).copied().unwrap_or_default()
    }
    /// Calculate the next `SlotRange` for an `id` given its `is_bulky`.
    fn calc_weight(&mut self) {
        let mut last = 0;
        for id in self.sorted.iter().copied() {
            let slots_used = self
                .items
                .get(id)
                .map(|item| item.is_bulky() as usize + 1)
                .unwrap_or(0);
            last += slots_used;
            let range = if last > self.max_size {
                SlotRange::Encumbered
            } else if slots_used > 1 {
                SlotRange::Double(last)
            } else {
                SlotRange::Single(last)
            };
            self.weight.insert(id, range);
        }
    }
    /// Inserts an item `name` into the `sorted` list.
    fn add_sorted(&mut self, id: usize, name: String) {
        let i = self
            .sorted
            .iter()
            .flat_map(|i| self.items.get(*i))
            .position(|item| matches!(item.name.cmp(&name), Ordering::Greater))
            .unwrap_or(self.sorted.len());
        if i == self.sorted.len() {
            self.sorted.push(id);
        } else {
            self.sorted.insert(i, id);
        }
    }
}

impl From<Vec<Item>> for Inventory {
    fn from(value: Vec<Item>) -> Self {
        let mut inv = Self {
            max_size: 10,
            ..Default::default()
        };
        for item in value {
            inv.add(item)
        }
        inv
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum SlotRange {
    Single(usize),
    Double(usize),
    Encumbered,
}

impl SlotRange {
    /// Gives the largest index or `None` if encumbered.
    pub fn largest(&self) -> Option<usize> {
        match self {
            SlotRange::Single(x) => Some(*x),
            SlotRange::Double(x) => Some(*x),
            SlotRange::Encumbered => None,
        }
    }
}

impl Default for SlotRange {
    fn default() -> Self {
        Self::Single(0)
    }
}

impl IntoView for SlotRange {
    fn into_view(self) -> View {
        match self {
            SlotRange::Single(x) => x.into_view(),
            SlotRange::Double(to) => format!("{} - {to}", to - 1).into_view(),
            SlotRange::Encumbered => view! {
                <div class= "fill-red-500 w-4" inner_html=icons::WEIGHT />
            }
            .into_view(),
        }
    }
}
