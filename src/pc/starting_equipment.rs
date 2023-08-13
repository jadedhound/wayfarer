use crate::items::simple::SUNDRIES;
use crate::items::weapons::WARHAMMER;
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
    [WARHAMMER.into()]
        .into_iter()
        // Add sundries
        .chain(sundries(rand))
        .collect()
}

/// Starting equipment for a warrior archetype.
pub(super) fn mage(rand: &mut Rand) -> Vec<Item> {
    [tome::ADHERE.into()]
        .into_iter()
        // Add sundries
        .chain(sundries(rand))
        .collect()
}
