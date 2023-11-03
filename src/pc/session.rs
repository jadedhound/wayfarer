use leptos::*;

use super::class::level::ClassLevel;
use super::Ability;
use crate::icons;
use crate::pc::AbiScores;
use crate::utils::index_map::IndexMap;
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
    pub inv_slots: IndexMap<SlotRange>,
    pub empty_inv_slots: usize,
    pub is_encumbered: bool,
    pub cast_divine: u8,
    pub cast_arcane: u8,
    pub sorted_inv: Vec<usize>,
}

impl RwUtils for Session {}

#[derive(Clone, Copy)]
pub enum SlotRange {
    Single(usize),
    Double(usize, usize),
    Encumbered,
}

impl SlotRange {
    /// Gives the largest index or `None` if encumbered.
    pub fn largest(&self) -> Option<usize> {
        match self {
            SlotRange::Single(x) => Some(*x),
            SlotRange::Double(_, x) => Some(*x),
            SlotRange::Encumbered => None,
        }
    }
}

impl Default for SlotRange {
    fn default() -> Self {
        Self::Single(0)
    }
}

impl IntoView for SlotRange {
    fn into_view(self) -> View {
        match self {
            SlotRange::Single(x) => x.into_view(),
            SlotRange::Double(from, to) => format!("{from} - {to}").into_view(),
            SlotRange::Encumbered => view! {
                <div class= "fill-red-500 w-4" inner_html=icons::WEIGHT />
            }
            .into_view(),
        }
    }
}
