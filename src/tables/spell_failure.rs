use crate::rand::Rand;

pub fn spell_failure() -> (&'static str, &'static str) {
    Rand::with(|rand| {
        let (affects, tbl) = rand.pick(&AFFECTS);
        let effect = rand.pick(tbl);
        (affects, effect)
    })
}

const AFFECTS: [(&str, &[&str]); 4] = [
    ("Wild magic surrounds YOU.", &YOU),
    ("Wild magic surges towards your ENEMIES.", &ENEMIES),
    ("Wild magic engulfes your ALLIES.", &ALLIES),
    ("Wild magic washes into the ENVIRONMENT.", &ENVIRONMENT),
];

const YOU: [&str; 6] = [
    "Flammable belongings in the inventory catch on fire.",
    "Turn into a pot plant. 1 HP.",
    "Turn into a great eagle.",
    "You randomly use another spell from the entire spell list.",
    "You begin to float at a rate of 10 ft. per round.",
    "A third eye opens on your forehead granting you darkvision for the next 10 mins.",
];

const ENEMIES: [&str; 6] = [
    "All enemies within 30 ft. burst into flames for 1 min. They take 1d4 per round but deal 1d8 damage to all nearby.",
    "All enemies become frightened of you. Morale check or flee.",
    "All creatures. become invisible, until they do an action.",
    "All creatures grow in size, their attacks do 1 die size higher. 1d4 added HP.",
    "Gain vulnerability to bludgeoning damage. ",
    "All dead creatures come back with 2 HP.",
];

const ALLIES: [&str; 6] = [
    "30 ft. fireball spell detonates around you. 1d8 fire damage to all.",
    "Roll 1d10, creatures within 30 ft. become that age for 1 hour.",
    "30 ft. healing circle centred on you. 1d8 healing to all.",
    "All allies are teleported to the caster and are unable to move for till the next round.",
    "An ally becomes a lightning conduit, they can choose 3 other creatures to bounce lightning of. 1d8 damage.",
    "You and your allies all swap bodies for the next 10 mins. Swap devices.",
];

const ENVIRONMENT: [&str; 6] = [
    "An odd mechanical creature is summoned. Roll for reaction.",
    "A force wall is summoned at a random point. 10 ft. impervious to all damage.",
    "A poisonous cloud 20 ft., 1 HP per round for 5 rounds.",
    "All weapons turn into various food till the next day.",
    "All creatures are teleported to the astral plane for 1 min. You can fly through walls and reappear where you are.",
    "Magical silence descends on the space almost permanently.",
];
