use super::ItemRef;
use crate::items::ItemPropRef as Prop;
use crate::utils::turns::Turns;

pub(super) const PRICES: [u32; 5] = [25, 50, 200, 400, 1000];

// -----------------------------------
// TIER 1
// -----------------------------------

const ACID_VIAL: ItemRef = ItemRef::new(
    "acid vial",
    PRICES[0],
    &[Prop::Usable(
        "the liquid in this vial bubbles ominously; best keep it in a secure place",
    )],
);
const BOTTLE_GREASE: ItemRef = ItemRef::new(
    "bottle of grease",
    PRICES[0],
    &[Prop::Usable(
        "a bottle of thick flammable slippery liquid; has enough to cover a 10 ft square area",
    )],
);
const CRACKER_NUTS: ItemRef = ItemRef::new(
    "cracker nuts",
    PRICES[0],
    &[Prop::Usable(
        "when thrown, creates a loud bang at the point of impact",
    )],
);
const GLUE: ItemRef = ItemRef::new(
    "glue",
    PRICES[0],
    &[Prop::Usable(
        "bond two objects by holding them together for 2 mins",
    )],
);
const VANISHING_POWDER: ItemRef = ItemRef::new(
    "vanishing powder",
    PRICES[0],
    &[Prop::Usable(
        "throw this powder to create a smoke veil large enough to shroud one person",
    )],
);

// -----------------------------------
// TIER 2
// -----------------------------------

const BOTTLE_BREATH: ItemRef = ItemRef::new(
    "bottled breath",
    PRICES[1],
    &[
        Prop::Usable("you don't need to breathe while you hold this breath"),
        Prop::Duration(Turns::hour()),
    ],
);
const CALMING_BREW: ItemRef = ItemRef::new(
    "calming brew",
    PRICES[1],
    &[
        Prop::Usable("the imbiber becomes serenely calm"),
        Prop::Duration(Turns::hour()),
    ],
);
const SCHOLARS_DRAUGHT: ItemRef = ItemRef::new(
    "scholar's draught",
    PRICES[1],
    &[Prop::Usable("regain 1d8 guard")],
);

#[rustfmt::skip]
pub const ITEMS: [&ItemRef; 8] = [
    &ACID_VIAL,
    &BOTTLE_GREASE,
    &CRACKER_NUTS,
    &GLUE,
    &VANISHING_POWDER,
    &BOTTLE_BREATH,
    &CALMING_BREW,
    &SCHOLARS_DRAUGHT,
];
