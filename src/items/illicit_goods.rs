use super::item_prop::ItemPropRef as Prop;
use super::ItemRef;
use crate::utils::counter::Counter;
use crate::utils::turns::Turns;

const ANISEED_VIAL: ItemRef = ItemRef::new(
    "aniseed vial",
    5,
    &[Prop::Usable(
        "A pungent extract that will disorientate scent based trackers",
    )],
);
const BLOOD_SEEKER_VIAL: ItemRef = ItemRef::new(
    "blood seeker vial",
    100,
    &[
        Prop::Effect("fill this vial with a targets blood to use"),
        Prop::Usable("ascertain the general location of the blood source"),
        Prop::Count(Counter::new(5)),
    ],
);
const CALTROPS: ItemRef = ItemRef::new(
    "caltrops",
    10,
    &[Prop::Usable(
        "cover a 10 ft. square, impeding movement or dealing damage",
    )],
);
const FAKE_JEWELS: ItemRef = ItemRef::simple("fake jewels", 50);
const LISTENING_CONE: ItemRef = ItemRef::new(
    "listening cone",
    20,
    &[
        Prop::Usable("listen through solid surfaces"),
        Prop::Count(Counter::new(3)),
    ],
);
const LOADED_DICE: ItemRef = ItemRef::simple("loaded dice (full set)", 5);
const MINDPIERCING_ARROW: ItemRef = ItemRef::new(
    "mindpiercing arrow",
    super::fletcher::PRICES[2],
    &super::fletcher::special_arrow(
        "deals psychic damage; if the target is reduced to 0 they remain in a brain addled state (dumb and mute) for 1d4 weeks",
    ),
);
const ODORLESS_POISON: ItemRef = ItemRef::new(
    "odorless poison",
    100,
    &[Prop::Usable(
        "probably best to hide this is someone else's drink",
    )],
);
const SWAMP_FEVER_VIAL: ItemRef = ItemRef::new(
    "swamp fever vial",
    super::alchemist::PRICES[1],
    &[Prop::Usable("having imbibed this potion, you begin to lash out against everything in an uncontrollable rage"), Prop::Duration(Turns::hour())],
);

pub const ITEMS: [&ItemRef; 9] = [
    &ANISEED_VIAL,
    &BLOOD_SEEKER_VIAL,
    &CALTROPS,
    &FAKE_JEWELS,
    &LISTENING_CONE,
    &LOADED_DICE,
    &MINDPIERCING_ARROW,
    &ODORLESS_POISON,
    &SWAMP_FEVER_VIAL,
];
