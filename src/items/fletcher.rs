use super::item_prop::ItemPropRef as Prop;
use super::ItemRef;
use crate::utils::counter::Counter;

pub(super) const PRICES: [u32; 3] = [30, 60, 120];

pub(super) const fn special_arrow(usable: &'static str) -> [Prop; 2] {
    [Prop::Usable(usable), Prop::Count(Counter::new(3))]
}

const FLAMING_ARROW: ItemRef = ItemRef::new(
    "flaming arrow",
    PRICES[0],
    &special_arrow("deals fire damage"),
);
const ICE_ARROW: ItemRef =
    ItemRef::new("ice arrow", PRICES[0], &special_arrow("deals frost damage"));
const KNOCKDOWN_ARROW: ItemRef = ItemRef::new(
    "knockdown arrow",
    PRICES[1],
    &special_arrow("knock prone a medium or smaller creature; deals 1d4 damage"),
);
const PINDOWN_ARROW: ItemRef = ItemRef::new(
    "pindown arrow",
    PRICES[1],
    &special_arrow("halves the movement speed of a medium or smaller creature; deals 1d4 damage"),
);
const TRICKSHOT_ARROW: ItemRef = ItemRef::new(
    "trickshot arrow",
    PRICES[1],
    &special_arrow("this arrow responds to your will and can change its flight path while mid-air"),
);
const DELAYED_ARROW: ItemRef = ItemRef::new(
    "delayed arrow",
    PRICES[2],
    &[
        Prop::Usable("freezes in mid-air once fired, staying as such while the user wills it"),
        Prop::Concentration,
        Prop::Count(Counter::new(3)),
    ],
);
const PHASE_ARROW: ItemRef = ItemRef::new(
    "phase arrow",
    PRICES[2],
    &special_arrow("phases through the first 20 ft of space"),
);
pub const ITEMS: [&ItemRef; 7] = [
    &FLAMING_ARROW,
    &ICE_ARROW,
    &KNOCKDOWN_ARROW,
    &PINDOWN_ARROW,
    &TRICKSHOT_ARROW,
    &DELAYED_ARROW,
    &PHASE_ARROW,
];
