use super::{BuffPropRef, BuffRef};
use crate::utils::counter::Counter;

const fn rest(effect: &'static str, count: usize) -> [BuffPropRef; 4] {
    [
        BuffPropRef::Class,
        BuffPropRef::Count(Counter::full(count)),
        BuffPropRef::Rest,
        BuffPropRef::Effect(effect),
    ]
}

const fn rally(effect: &'static str) -> [BuffPropRef; 4] {
    [
        BuffPropRef::Class,
        BuffPropRef::Count(Counter::full(1)),
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
        BuffPropRef::Count(Counter::full(count)),
        BuffPropRef::Effect(effect),
    ]
}

#[rustfmt::skip]
pub mod fighter {
    use super::*;

    const T1_PROPS: [BuffPropRef; 4] = rally("as a quick action parry a melee attack targetting you");
    pub const T1: BuffRef = BuffRef::new("parry", &T1_PROPS);

    const T2_PROPS: [BuffPropRef; 2] = passive("you can make two attacks as a action");
    pub const T2: BuffRef = BuffRef::new("multiattack", &T2_PROPS);

    const T3_PROPS: [BuffPropRef; 4] = rest("do an action as a quick action", 3);
    pub const T3: BuffRef = BuffRef::new("action surge", &T3_PROPS);

    // -----------------------------------
    // OPTIONAL
    // -----------------------------------

    const ENRAGE_PROPS: [BuffPropRef; 4] = rest("as a quick action enrage yourself, while enraged, deal and suffer [ level ] additional damage", 3);
    pub const ENRAGE: BuffRef = BuffRef::new("enrage", &ENRAGE_PROPS);

    const CHARGE_PROPS: [BuffPropRef; 2] = passive("move [ your speed ] as a quick action");
    pub const CHARGE: BuffRef = BuffRef::new("charge", &CHARGE_PROPS);

    const ON_THE_HUNT_PROPS: [BuffPropRef; 2] = passive("take a gruesome trophy from a slayen foe, similar creatures become afraid of you for 1 hour");
    pub const ON_THE_HUNT: BuffRef = BuffRef::new("on the hunt", &ON_THE_HUNT_PROPS);
}

#[rustfmt::skip]
pub mod rogue {
    use super::*;
    
    const T1_PROPS: [BuffPropRef; 2] = passive("attacks from stealth deal maximum damage");
    pub const T1: BuffRef = BuffRef::new("unseen blade", &T1_PROPS);

    const T2_PROPS: [BuffPropRef; 4] = rest("choose to half non-magical damage taken from a single source", 3);
    pub const T2: BuffRef = BuffRef::new("uncanny dodge", &T2_PROPS);

    const T3_PROPS: [BuffPropRef; 2] = passive("all dexterity checks can be done as a quick action");
    pub const T3: BuffRef = BuffRef::new("quick fingers", &T3_PROPS);

    // -----------------------------------
    // OPTIONAL
    // -----------------------------------

    const HUNTERS_MARK_PROPS: [BuffPropRef; 4] = rest("you are able to magically track a target within sight for 1 hour, no barrier can stop this sight", 1);
    pub const HUNTERS_MARK: BuffRef = BuffRef::new("hunter's mark", &HUNTERS_MARK_PROPS);

    const MAGE_HAND_PROPS: [BuffPropRef; 2] = passive("summon a spectral hand, as an action it can manipulate up to 5 lbs. at a maximum of 30 ft. away");
    pub const MAGE_HAND: BuffRef = BuffRef::new("mage hand", &MAGE_HAND_PROPS);
}

#[rustfmt::skip]
pub mod mage {
    use super::*;
    
    const T1_PROPS: [BuffPropRef; 2] = passive("you can create novice arcane scrolls");
    pub const T1: BuffRef = BuffRef::new("novice arcane scrolls", &T1_PROPS);

    const T2_PROPS: [BuffPropRef; 2] = passive("you can create expert arcane scrolls");
    pub const T2: BuffRef = BuffRef::new("expert arcane scrolls", &T2_PROPS);

    const T3_PROPS: [BuffPropRef; 2] = passive("you can create master arcane scrolls");
    pub const T3: BuffRef = BuffRef::new("master arcane scrolls", &T3_PROPS);

    // -----------------------------------
    // OPTIONAL
    // -----------------------------------

    const METAMAGIC_PROPS: [BuffPropRef; 4] = rest("make a spell subtle, quick or twinned", 3);
    pub const METAMAGIC: BuffRef = BuffRef::new("metamagic", &METAMAGIC_PROPS);

    const FIREBOLT_PROPS: [BuffPropRef; 2] = passive("hurl a mote of fire 30 ft. as an action, dealing 1d8 on impact");
    pub const FIREBOLT: BuffRef = BuffRef::new("firebolt", &FIREBOLT_PROPS);

    const FIND_FAMILIAR_PROPS: [BuffPropRef; 2] = passive("summon a familiar that is intelligent and can speak, requires 1 day to resummon");
    pub const FIND_FAMILIAR: BuffRef = BuffRef::new("find familiar", &FIND_FAMILIAR_PROPS);

    const MAGECRAFT_PROPS: [BuffPropRef; 2] = passive("minor magical effect: transfer heat, visual or auditory illusion or soil/clean as object");
    pub const MAGECRAFT: BuffRef = BuffRef::new("magecraft", &MAGECRAFT_PROPS);
}

#[rustfmt::skip]
pub mod cleric {
    use super::*;

    const T1_PROPS: [BuffPropRef; 2] = passive("you can create novice divine scrolls");
    pub const T1: BuffRef = BuffRef::new("novice divine scrolls", &T1_PROPS);

    const T2_PROPS: [BuffPropRef; 2] = passive("you can create expert divine scrolls");
    pub const T2: BuffRef = BuffRef::new("expert divine scrolls", &T2_PROPS);

    const T3_PROPS: [BuffPropRef; 2] = passive("you can create master divine scrolls");
    pub const T3: BuffRef = BuffRef::new("master divine scrolls", &T3_PROPS);

    // -----------------------------------
    // OPTIONAL
    // -----------------------------------

    const TURN_UNDEAD_PROPS: [BuffPropRef; 4] = rest("[ level ] HD worth of undead flee as an action", 3);
    pub const TURN_UNDEAD: BuffRef = BuffRef::new("TURN_UNDEAD", &TURN_UNDEAD_PROPS);

    const BULWARK_OF_FAITH_PROPS: [BuffPropRef; 4] = rest("kneel in prayer, while praying, creating a 10 ft. semi-circle divine shield in front of you", 1);
    pub const BULWARK_OF_FAITH: BuffRef = BuffRef::new("bulwark of faith", &BULWARK_OF_FAITH_PROPS);

    const SMITE_PROPS: [BuffPropRef; 3] = manual("deal [ guard ] damage as a quick action; roll a d4 at dawn, on a 1 this ability recharges", 1);
    pub const SMITE: BuffRef = BuffRef::new("smite", &SMITE_PROPS);

}
