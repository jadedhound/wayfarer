use crate::items::armour::{BRIGANDINE, CHAUSSES, LEGGINGS, MAGE_ROBE, SHIELD};
use crate::items::simple::{SUNDRIES, TOOLS_T1};
use crate::items::weapons::{SWORD, WARHAMMER};
use crate::items::{tome, Item};
use crate::rand::Rand;

/// Randomly pick 3 sundries.
fn sundries(rand: &mut Rand) -> impl Iterator<Item = Item> + '_ {
    vec![(); 3]
        .into_iter()
        .map(|_| (*rand.pick(&SUNDRIES)).into())
}

/// Starting equipment for a warrior archetype.
pub(super) fn warrior(rand: &mut Rand) -> Vec<Item> {
    [
        TOOLS_T1.into(),
        CHAUSSES.into(),
        BRIGANDINE.into(),
        SHIELD.into(),
        WARHAMMER.into(),
    ]
    .into_iter()
    // Add sundries
    .chain(sundries(rand))
    .enumerate()
    .map(|(i, mut x)| {
        x.id = i as u32;
        x
    })
    .collect()
}

/// Starting equipment for a warrior archetype.
pub(super) fn mage(rand: &mut Rand) -> Vec<Item> {
    [
        TOOLS_T1.into(),
        LEGGINGS.into(),
        MAGE_ROBE.into(),
        tome::ADHERE.into(),
    ]
    .into_iter()
    // Add sundries
    .chain(sundries(rand))
    .enumerate()
    .map(|(i, mut x)| {
        x.id = i as u32;
        x
    })
    .collect()
}
