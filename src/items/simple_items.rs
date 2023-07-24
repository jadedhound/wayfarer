use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::SimpleItem as SI;

pub static SUNDRIES: Lazy<HashMap<String, SI>> = Lazy::new(|| {
    vec![
        SI::new("50 ft. rope", 10),
        SI::new("10 ft. iron chain", 10),
        SI::new("iron manacles", 10),
        SI::new("hooded lantern", 50),
        SI::new("sack of marbles", 10),
        SI::new("personal tent", 50),
        SI::new("sack", 1),
        SI::new("crowbar", 10),
        SI::new("hammer", 50),
        SI::new("chisel", 10),
        SI::new("lockpicks", 100),
        SI::new("horn", 10),
        SI::new("needle", 10),
        SI::new("thread", 10),
        SI::new("shovel", 10),
        SI::new("hourglass", 300),
        SI::new("mirror", 200),
        SI::new("fishing net", 10),
        SI::new("grappling hook", 10),
        SI::new("bell", 20),
        SI::new("iron file", 10),
        SI::new("bucket", 5),
    ]
    .into_iter()
    .map(|item| (item.name.clone(), item))
    .collect()
});
