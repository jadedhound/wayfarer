use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// One turn is 10 mins.
const TURNS_IN_DAY: u64 = 24 * 6;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Turns(pub u64);

impl Turns {
    pub const fn new(time: u64) -> Self {
        Self(time)
    }
    pub fn next_day(&mut self) {
        self.0 += TURNS_IN_DAY - self.turns()
    }
    pub fn change_by(&mut self, amount: i64) {
        self.0 = (self.0 as i64 + amount) as u64
    }
    pub fn set(&mut self, time_ref: &Self) {
        self.0 += time_ref.0;
    }
    pub fn is_expired(&self, time_ref: u64) -> bool {
        self.0 <= time_ref
    }

    /// Absolute time difference with a given `other`.
    pub fn diff(&self, other: Self) -> Self {
        Self(u64::abs_diff(self.0, other.0))
    }
    pub fn days(&self) -> u64 {
        self.0 / TURNS_IN_DAY
    }
    pub fn turns(&self) -> u64 {
        self.0 % TURNS_IN_DAY
    }
}

impl Display for Turns {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let days = self.days();
        let turns = self.turns();
        if days != 0 && turns != 0 {
            write!(f, "{days} days {turns} turns")
        } else if days != 0 {
            write!(f, "{days} days")
        } else if turns != 0 {
            write!(f, "{turns} turns")
        } else {
            write!(f, "instant")
        }
    }
}
