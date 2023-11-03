use super::ItemRef;
use crate::buffs::{BuffPropRef as BuffProp, BuffRef};
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

const BOTTLE_BREATH: ItemRef =
    ItemRef::new("bottled breath", PRICES[1], &[Prop::Buff(LONGBREATH_BUFF)]);
const LONGBREATH_BUFF: BuffRef = BuffRef::new(
    "long breath",
    &[
        BuffProp::Effect("you don't need to breathe while you hold this breath"),
        BuffProp::Duration(Turns::hour()),
    ],
);
const CALMING_BREW: ItemRef = ItemRef::new("calming brew", PRICES[1], &[Prop::Buff(SERENELY_CALM)]);
const SERENELY_CALM: BuffRef = BuffRef::new(
    "serenely calm",
    &[
        BuffProp::Effect("having imbibed this potion, very little seems to anger or stress you"),
        BuffProp::Duration(Turns::hour()),
    ],
);
const POTION_OF_LONGSTRIDER: ItemRef = ItemRef::new(
    "potion of longstrider",
    PRICES[1],
    &[Prop::Buff(LONGSTRIDER_BUFF)],
);
const LONGSTRIDER_BUFF: BuffRef = BuffRef::new(
    "longstrider",
    &[
        BuffProp::Effect("your speed is doubled"),
        BuffProp::Duration(Turns::hour()),
    ],
);
const SCHOLARS_DRAUGHT: ItemRef = ItemRef::new(
    "scholar's draught",
    PRICES[1],
    &[Prop::Usable("regain 1d8 guard")],
);

#[rustfmt::skip]
pub const BUFFS: [&BuffRef; 3] = [
    &LONGSTRIDER_BUFF, &LONGBREATH_BUFF, 
    &SERENELY_CALM
];
#[rustfmt::skip]
pub const ITEMS: [&ItemRef; 9] = [
    &ACID_VIAL,
    &BOTTLE_GREASE,
    &CRACKER_NUTS,
    &GLUE,
    &VANISHING_POWDER,
    &BOTTLE_BREATH,
    &CALMING_BREW,
    &POTION_OF_LONGSTRIDER,
    &SCHOLARS_DRAUGHT,
];
