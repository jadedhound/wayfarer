use super::{ItemPropRef as Prop, ItemRef};
use crate::buffs::{BuffPropRef as BuffProp, BuffRef as Buff};
use crate::utils::turns::Turns;

const fn free(name: &'static str, props: &'static [Prop]) -> ItemRef {
    ItemRef::new(name, 0, props)
}

pub const ARCANE_ARROW: ItemRef = free(
    "spell: arcane arrow",
    &[
        Prop::Usable(
            "the bolt travels in a straight line, dealing 1d4 damage to all creatures in its path",
        ),
        Prop::Range(100),
    ],
);
const ATTRACT: ItemRef = free("spell: attract", &[Prop::Buff(ATTRACT_BUFF)]);
const ATTRACT_BUFF: Buff = Buff::new(
    "arcane: attract",
    &[
        BuffProp::Effect(
            "two objects are magnetically attracted to one another if they come within 10 ft",
        ),
        BuffProp::Duration(Turns::hour()),
    ],
);
pub const AUDITORY_ILLUSION: ItemRef = free(
    "spell: auditory illusion",
    &[Prop::Buff(AUDITORY_ILLUSION_BUFF)],
);
const AUDITORY_ILLUSION_BUFF: Buff = Buff::new(
    "arcane: auditory illusion",
    &[
        BuffProp::Effect(
            "create an auditory illusion that seems to come from a direction of your choice",
        ),
        BuffProp::Duration(Turns::one()),
    ],
);
const COMPREHEND: ItemRef = free(
    "spell: comprehend",
    &[
        Prop::Usable("you are fluent in all languages"),
        Prop::Concentration,
    ],
);
pub const FEATHER_FALL: ItemRef = free(
    "spell: feather fall",
    &[
        Prop::Usable("any chosen creatures within sight fall at a slow pace"),
        Prop::Concentration,
    ],
);
const LEAP: ItemRef = free(
    "spell: leap",
    &[
        Prop::Usable("a creature of your choice can leap [ level ] x 10 ft in the air"),
        Prop::Concentration,
    ],
);
const LIGHT: ItemRef = free("spell: light", &[Prop::Buff(LIGHT_BUFF)]);
const LIGHT_BUFF: Buff = Buff::new(
    "arcane: light",
    &[
        BuffProp::Effect("an object you've touched sheds 30 ft. of light around itself"),
        BuffProp::Duration(Turns::hour()),
    ],
);
const VISUAL_ILLUSION: ItemRef = free(
    "spell: visual illusion",
    &[Prop::Buff(VISUAL_ILLUSION_BUFF)],
);
const VISUAL_ILLUSION_BUFF: Buff = Buff::new(
    "arcane: visual illusion",
    &[
        BuffProp::Effect(
            "a silent, immobile illusion of your choice appears, up to the size of a bedroom",
        ),
        BuffProp::Duration(Turns::one()),
    ],
);

pub const ITEMS: [&ItemRef; 8] = [
    &ARCANE_ARROW,
    &ATTRACT,
    &AUDITORY_ILLUSION,
    &COMPREHEND,
    &FEATHER_FALL,
    &LEAP,
    &LIGHT,
    &VISUAL_ILLUSION,
];
pub const BUFFS: [&Buff; 4] = [
    &ATTRACT_BUFF,
    &AUDITORY_ILLUSION_BUFF,
    &LIGHT_BUFF,
    &VISUAL_ILLUSION_BUFF,
];
