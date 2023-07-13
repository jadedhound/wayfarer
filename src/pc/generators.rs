use crate::items::{Gear, Item, SimpleItem, SUNDRIES};
use crate::rand::Rand;

fn create_held(name: &'static str) -> Item {
    Item::Held(Gear::create(None, name.into(), None, 0).unwrap())
}

fn create_armour(name: &'static str) -> Item {
    Item::Armour(Gear::create(None, name.into(), None, 0).unwrap())
}

pub fn gen_warrior_inv(rand: &mut Rand) -> Vec<Item> {
    // Randomly pick 3 sundries
    let sundries: Vec<&SimpleItem> = SUNDRIES.values().collect();
    let mut inv: Vec<Item> = vec![(); 3]
        .into_iter()
        .map(|_| Item::Simple(rand.pick(&sundries).clone()))
        .collect();

    // Weapons and armour
    inv.extend_from_slice(&[
        create_held("sword"),
        create_held("shield"),
        create_armour("brigandine"),
        create_armour("chausses"),
    ]);
    inv
}
