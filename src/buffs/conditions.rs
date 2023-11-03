use super::{BuffPropRef as Prop, BuffRef};
use crate::pc::Ability;

const BLIND_PROP: [Prop; 2] = [Prop::Debuff, Prop::Effect("you are unable to see")];
const BLIND: BuffRef = BuffRef::new("blinded", &BLIND_PROP);

const DEAF_PROP: [Prop; 2] = [Prop::Debuff, Prop::Effect("you are unable to hear")];
const DEAF: BuffRef = BuffRef::new("deafened", &DEAF_PROP);

const FRIGHT_PROP: [Prop; 2] = [
    Prop::Debuff,
    Prop::Effect("the source of your fright deals maximum damage"),
];
const FRIGHT: BuffRef = BuffRef::new("frightened", &FRIGHT_PROP);

const STUNNED_PROP: [Prop; 2] = [Prop::Debuff, Prop::ScoreOverride(Ability::Guard, 0)];
const STUNNED: BuffRef = BuffRef::new("stunned", &STUNNED_PROP);

const ENCUMBERED_PROP: [Prop; 2] = [Prop::Debuff, Prop::ScoreOverride(Ability::Guard, 0)];
pub const ENCUMBERED: BuffRef = BuffRef::new("encumbered", &ENCUMBERED_PROP);

const INVISIBLE_PROP: [Prop; 1] = [Prop::Effect(
    "you are invisible while stationary and hard to see while moving",
)];
const INVISIBLE: BuffRef = BuffRef::new("invisible", &INVISIBLE_PROP);

pub const ALL: [&BuffRef; 5] = [&BLIND, &DEAF, &FRIGHT, &STUNNED, &INVISIBLE];
