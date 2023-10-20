use crate::buffs::{BuffPropRef, BuffRef};
use crate::utils::counter::Counter;

const fn rest(effect: &'static str, count: usize) -> [BuffPropRef; 4] {
    [
        BuffPropRef::Class,
        BuffPropRef::Count(Counter::new(count)),
        BuffPropRef::Rest,
        BuffPropRef::Effect(effect),
    ]
}

const fn rally(effect: &'static str) -> [BuffPropRef; 4] {
    [
        BuffPropRef::Class,
        BuffPropRef::Count(Counter::new(1)),
        BuffPropRef::Rally,
        BuffPropRef::Effect(effect),
    ]
}

const fn passive(effect: &'static str) -> [BuffPropRef; 2] {
    [BuffPropRef::Class, BuffPropRef::Effect(effect)]
}

const fn manual(effect: &'static str, count: usize) -> [BuffPropRef; 3] {
    [
        BuffPropRef::Class,
        BuffPropRef::Count(Counter::new(count)),
        BuffPropRef::Effect(effect),
    ]
}

pub(super) const FIGHTER_MAIN: [&BuffRef; 3] = [
    &BuffRef::new(
        "parry",
        &rally("as a quick action parry a melee attack targetting you"),
    ),
    &BuffRef::new(
        "multiattack",
        &passive("you can make two attacks as a action"),
    ),
    &BuffRef::new("action surge", &rest("do an action as a quick action", 3)),
];

pub(super) const FIGHTER_OPTIONAL: [&BuffRef; 3] = [
    &BuffRef::new(
        "enrage", 
        &rest("as a quick action enrage yourself, while enraged, deal and suffer [ level ] additional damage", 3)
    ), 
    &BuffRef::new("charge", &passive("move [ your speed ] as a quick action"))   , 
    &BuffRef::new("on the hunt", &passive("take a gruesome trophy from a slayen foe, similar creatures become afraid of you for 1 hour"))    
];

pub(super) const ROGUE_MAIN: [&BuffRef; 3] = [
    &BuffRef::new(
        "unseen blade",
        &passive("attacks from stealth deal maximum damage"),
    ),
    &BuffRef::new(
        "uncanny dodge",
        &rest(
            "choose to half non-magical damage taken from a single source",
            3,
        ),
    ),
    &BuffRef::new(
        "quick fingers",
        &passive("all dexterity checks can be done as a quick action"),
    ),
];

pub(super) const ROGUE_OPTIONAL: [&BuffRef; 2] = [

&BuffRef::new("hunter's mark", &rest("you are able to magically track a target within sight for 1 hour, no barrier can stop this sight", 1)),

&BuffRef::new("mage hand", &passive("summon a spectral hand, as an action it can manipulate up to 5 lbs. at a maximum of 30 ft. away")),
    ];

pub(super) const MAGE_MAIN: [&BuffRef; 3] = [
    
    &BuffRef::new("novice arcane spells", &passive("you can use components to cast novice arcane spells")),
    
    &BuffRef::new("expert arcane spells", &passive("you can use components to cast expert arcane spells")),
    
    &BuffRef::new("master arcane spells", &passive("you can use components to cast master arcane spells")),

    ];
pub(super) const MAGE_OPTIONAL: [&BuffRef; 4] = [
    
    &BuffRef::new("metamagic", &rest("make a spell subtle, quick or twinned", 3)),
    
    &BuffRef::new("firebolt", &passive("hurl a mote of fire 30 ft. as an action, dealing 1d8 on impact")),
    
    &BuffRef::new("find familiar", &passive("summon a familiar that is intelligent and can speak, requires 1 day to resummon")),
    
    &BuffRef::new("magecraft", &passive("minor magical effect: transfer heat, visual or auditory illusion or soil/clean as object")),
    ];
pub(super) const CLERIC_MAIN: [&BuffRef; 3] = [
    
    &BuffRef::new("novice divine spells", &passive("you can call upon the gods to cast novice divine spells")),
    
    &BuffRef::new("expert divine spells", &passive("you can call upon the gods to cast expert divine spells")),
    
    &BuffRef::new("master divine spells", &passive("you can call upon the gods to cast master divine spells")),
    ];
pub(super) const CLERIC_OPTIONAL: [&BuffRef; 3] = [
    
    &BuffRef::new("TURN_UNDEAD", &rest("[ level ] HD worth of undead flee as an action", 3)),
    
    &BuffRef::new("bulwark of faith", &rest("kneel in prayer, while praying, creating a 10 ft. semi-circle divine shield in front of you", 1)),
    
    &BuffRef::new("smite", &manual("deal [ guard ] damage as a quick action; roll a d4 at dawn, on a 1 this ability recharges", 1)),

    ];
