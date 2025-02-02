use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// One turn is 10 mins.
pub const TURNS_IN_DAY: u64 = 24 * 6;

#[derive(Serialize, Deserialize, Copy, Clone, Default, PartialEq)]
pub struct Turns(pub u64);

impl Turns {
    pub const fn new(days: u64, turns: u64) -> Self {
        Self(days * TURNS_IN_DAY + turns)
    }
    pub const fn one() -> Self {
        Self(1)
    }
    pub const fn hour() -> Self {
        Self(6)
    }
    pub fn add(&mut self, value: u64) {
        self.0 = self.0.wrapping_add(value);
    }
    pub fn sub(&mut self, value: u64) {
        self.0 = self.0.saturating_sub(value);
    }
    pub fn is_expired(&self, time_ref: u64) -> bool {
        self.0 <= time_ref
    }
    /// Absolute time difference with a given `other`.
    pub fn abs_diff(&self, other: Self) -> Self {
        Self(u64::abs_diff(self.0, other.0))
    }
    pub fn in_days(&self) -> u64 {
        self.0 / TURNS_IN_DAY
    }
    pub fn in_turns(&self) -> u64 {
        self.0 % TURNS_IN_DAY
    }
}

impl Display for Turns {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let days = self.in_days();
        let turns = self.in_turns();
        if days != 0 && turns != 0 {
            write!(f, "{days} days {turns} turns")
        } else if days != 0 {
            write!(f, "{days} days")
        } else {
            const SUFFIX_TEXT: [&str; 2] = ["turn", "turns"];
            let suffix = SUFFIX_TEXT[(turns > 1) as usize];
            write!(f, "{turns} {suffix}")
        }
    }
}
