use super::{ItemPropRef as Prop, ItemRef};
use crate::buffs::{BuffPropRef as BuffProp, BuffRef as Buff};
use crate::utils::turns::Turns;

const fn free(name: &'static str, props: &'static [Prop]) -> ItemRef {
    ItemRef::new(name, 0, props)
}

// -----------------------------------
// ITEMS
// -----------------------------------

pub const HOLY_WATER: ItemRef = ItemRef::new("holy water", 25, &[]);
pub const INCENSE: ItemRef = ItemRef::new("incense", 10, &[]);

// -----------------------------------
// SPELLS
// -----------------------------------

pub const AUGURY: ItemRef = free(
    "spell: augury",
    &[Prop::Effect("employ your divination tools to receive an omen (weal/woe) from the gods about a course of action, subsequent casts can fail")],
);
pub const BABBLE: ItemRef = free(
    "spell: babble",
    &[
        Prop::Effect(
            "a creature must loudly and clearly repeat everything you think, it is otherwise mute",
        ),
        Prop::Concentration,
    ],
);
pub const BANE: ItemRef = free(
    "spell: bane",
    &[
        Prop::Effect(
            "a target within sight must subtract 1d4 from all damage rolls and skill checks",
        ),
        Prop::Concentration,
    ],
);
pub const CHARM: ItemRef = free(
    "spell: charm",
    &[Prop::Buff(CHARM_BUFF), Prop::Range(30), Prop::Resist],
);
const CHARM_BUFF: Buff = Buff::new(
    "divine: charm",
    &[
        BuffProp::Effect("one creature treats you as a friend"),
        BuffProp::Duration(Turns::hour()),
    ],
);
pub const DEAFEN: ItemRef = free(
    "spell: deafen",
    &[
        Prop::Effect("all nearby creatures are deafened"),
        Prop::Concentration,
    ],
);
const GUIDING_BOLT: ItemRef = free(
    "spell: guiding bolt",
    &[
        Prop::Usable("the next attack against the chosen target deals maximum damage"),
        Prop::Range(60),
    ],
);
const HEALING_WORD: ItemRef = free(
    "spell: healing word",
    &[
        Prop::Usable("a creature within line of sight heals [ level ] x 1d8 guard"),
        Prop::Range(30),
    ],
);
pub const MESSAGE: ItemRef = free(
    "spell: message",
    &[
        Prop::Usable("send a brief psychic message to a known target and hear their reply"),
        Prop::Range(600),
    ],
);

pub const ITEMS: [&ItemRef; 10] = [
    &HOLY_WATER,
    &INCENSE,
    &AUGURY,
    &BABBLE,
    &BANE,
    &CHARM,
    &DEAFEN,
    &GUIDING_BOLT,
    &HEALING_WORD,
    &MESSAGE,
];
pub const BUFFS: [&Buff; 1] = [&CHARM_BUFF];
