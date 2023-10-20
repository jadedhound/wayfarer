use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub struct Counter {
    pub curr: usize,
    pub max: usize,
}

impl Counter {
    pub const fn new(count: usize) -> Self {
        Self {
            curr: count,
            max: count,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.curr == 0
    }
}
