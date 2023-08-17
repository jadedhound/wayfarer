use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use crate::utils::split_operator;

#[derive(Serialize, Deserialize, Clone, Copy, strum::Display, EnumCount, EnumIter)]
#[allow(clippy::upper_case_acronyms)]
pub enum PCStat {
    Stamina,
    Health,
    Inventory,
    Recipes,
    Speed,
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

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct StatArray([i32; PCStat::COUNT]);

impl StatArray {
    pub fn iter(&self) -> impl Iterator<Item = (PCStat, i32)> + '_ {
        PCStat::iter().zip(self.0.iter().copied())
    }
    pub fn merge(&mut self, other: Self) {
        for (stat, amount) in other.iter() {
            self.0[stat as usize] += amount
        }
    }
    pub fn get(&self, stat: PCStat) -> i32 {
        self.0[stat as usize]
    }
    pub fn get_mut(&mut self, stat: PCStat) -> &mut i32 {
        self.0.get_mut(stat as usize).unwrap()
    }
    pub fn set(&mut self, stat: PCStat, to: i32) {
        self.0[stat as usize] = to
    }
}

impl Display for StatArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let result = self
            .iter()
            .filter(|(_, num)| *num != 0)
            .map(|(stat, num)| {
                let (op, num) = split_operator(num);
                format!("{stat} {op}{num}")
            })
            .reduce(|mut acc, e| {
                acc.push_str(", ");
                acc.push_str(&e);
                acc
            })
            .unwrap_or_default();
        write!(f, "{result}")
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct StatArrBuilder([i32; PCStat::COUNT]);

#[allow(dead_code)]
impl StatArrBuilder {
    pub fn iter(&self) -> impl Iterator<Item = (PCStat, i32)> + '_ {
        PCStat::iter().zip(self.0.iter().copied())
    }

    pub const fn new() -> Self {
        Self([0; PCStat::COUNT])
    }

    pub const fn str(mut self, x: i32) -> Self {
        self.0[PCStat::STR.index()] += x;
        self
    }
    pub const fn dex(mut self, x: i32) -> Self {
        self.0[PCStat::DEX.index()] += x;
        self
    }
    pub const fn int(mut self, x: i32) -> Self {
        self.0[PCStat::INT.index()] += x;
        self
    }
    pub const fn cha(mut self, x: i32) -> Self {
        self.0[PCStat::CHA.index()] += x;
        self
    }
    pub const fn stam(mut self, x: i32) -> Self {
        self.0[PCStat::Stamina.index()] += x;
        self
    }
    pub const fn health(mut self, x: i32) -> Self {
        self.0[PCStat::Health.index()] += x;
        self
    }
    pub const fn inventory(mut self, x: i32) -> Self {
        self.0[PCStat::Inventory.index()] += x;
        self
    }
    pub const fn recipes(mut self, x: i32) -> Self {
        self.0[PCStat::Recipes.index()] += x;
        self
    }
    pub const fn speed(mut self, x: i32) -> Self {
        self.0[PCStat::Speed.index()] += x;
        self
    }
    pub const fn build(self) -> StatArray {
        StatArray(self.0)
    }
}
