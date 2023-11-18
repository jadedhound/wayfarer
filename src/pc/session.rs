use super::class::level::ClassLevel;
use super::Ability;
use crate::pc::AbiScores;
use crate::utils::rw_utils::RwUtils;

#[derive(Default)]
pub struct Session {
    // Collated scores from everything below.
    pub abi_scores: AbiScores,
    // Each score is only adjusted by one thing.
    pub isolated_scores: AbiScores,
    // All scores adjusted by buffs.
    pub buff_scores: AbiScores,
    // Buffs with score overrides.
    pub override_scores: Vec<(Ability, i32)>,
    // All scores adjusted by inventory items.
    pub inv_scores: AbiScores,
    pub level: ClassLevel,
    pub cast_divine: u8,
    pub cast_arcane: u8,
}

impl RwUtils for Session {}
