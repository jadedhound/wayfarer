use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};

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

impl PCStat {
    pub const fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct StatArray(pub [i32; PCStat::COUNT]);

impl StatArray {
    pub fn get(&self, stat: PCStat) -> i32 {
        self.0[stat as usize]
    }
    pub fn get_mut(&mut self, stat: PCStat) -> &mut i32 {
        self.0.get_mut(stat as usize).unwrap()
    }
}

impl From<Vec<(PCStat, i32)>> for StatArray {
    fn from(value: Vec<(PCStat, i32)>) -> Self {
        let mut def = StatArray::default();
        for (stat, by) in value {
            *def.get_mut(stat) = by
        }
        def
    }
}
