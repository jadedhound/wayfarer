use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::SimpleItem as SI;
use crate::utils::{char_count, CharHash};

pub static SUNDRY_CMPHASH: Lazy<HashMap<String, CharHash>> = Lazy::new(|| {
    SUNDRIES
        .iter()
        .map(|(name, _)| (name.to_owned(), char_count(name)))
        .collect()
});

pub static SUNDRIES: Lazy<HashMap<String, SI>> = Lazy::new(|| {
    vec![
        SI::new("25 ft. rope"),
        SI::new("10 ft. iron chain"),
        SI::new("iron manacles"),
        SI::new("hooded lantern"),
        SI::new("sack of marbles"),
        SI::new("10 ft. oilskin tarp"),
        SI::new("small barrel"),
        SI::new("iron crowbar"),
        SI::new("hammer and chisel"),
        SI::new("25 ft. of bandages"),
        SI::new("large horn"),
        SI::new("needle and thread"),
        SI::new("shovel"),
        SI::new("hourglass"),
        SI::new("steel mirror"),
        SI::new("fishing net"),
        SI::new("grappling hook"),
        SI::new("bell"),
        SI::new("iron file"),
    ]
    .into_iter()
    .map(|item| (item.name.clone(), item))
    .collect()
});
