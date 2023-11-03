use serde::{Deserialize, Serialize};

use crate::pc::class::PCClassRef;
use crate::utils::rw_utils::RwUtils;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PCBasic {
    pub name: String,
    pub class: PCClassRef,
}

impl RwUtils for PCBasic {}

#[rustfmt::skip]
pub const NAMES: [&str; 23] = [
    "Abigail","Emilia","Allison","Clara","Leah",
    "Myla","Ryanna","Valerie","Bram","Abram","Astin",
    "Bradyn","Cartus","Eric","Gavin","Han","Jax",
    "Jovan","Liam","Remus","Sebastion","Xander","Havy"
];
