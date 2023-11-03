use std::cmp;

use serde::{Deserialize, Serialize};

const EXP: [usize; 7] = [0, 500, 1500, 3000, 5500, 9500, 16000];

#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
pub struct ClassExp(usize);

impl ClassExp {
    pub fn get(&self) -> usize {
        self.0
    }
    pub fn change(&mut self, by: isize) {
        let new_value = usize::try_from(self.0 as isize + by).unwrap_or_default();
        self.0 = cmp::min(new_value, EXP[EXP.len() - 1])
    }
    pub fn level(&self) -> ClassLevel {
        let max_i = EXP.len() - 1;
        let mut level = max_i;
        for i in 0..max_i {
            if self.0 >= EXP[i] && self.0 < EXP[i + 1] {
                level = i + 1;
                break;
            }
        }
        ClassLevel(level)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct ClassLevel(usize);

impl ClassLevel {
    pub fn get(&self) -> usize {
        self.0
    }
    pub fn min_exp(&self) -> usize {
        EXP.get(self.0.saturating_sub(1)).copied().unwrap_or(EXP[0])
    }
    pub fn max_exp(&self) -> usize {
        EXP.get(self.0).copied().unwrap_or(EXP[EXP.len() - 1])
    }
}

impl Default for ClassLevel {
    fn default() -> Self {
        Self(1)
    }
}
