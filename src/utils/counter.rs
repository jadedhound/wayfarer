use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub struct Counter {
    pub curr: usize,
    pub max: usize,
}

impl Counter {
    pub const fn empty(count: usize) -> Self {
        Self {
            curr: 1,
            max: count,
        }
    }
    pub const fn full(count: usize) -> Self {
        Self {
            curr: count,
            max: count,
        }
    }
    pub fn decr(&mut self) {
        self.curr -= 1;
    }
    pub fn incr(&mut self) {
        self.curr += 1;
    }
    pub fn is_zero(&self) -> bool {
        self.curr == 0
    }
}
