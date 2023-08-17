use leptos::*;

use super::PC;
use crate::pc::pc_stat::StatArray;
use crate::svg;
use crate::utils::index_map::IndexMap;
use crate::utils::RwProvided;

#[derive(Clone)]
pub(super) struct PCSession {
    pub stats: StatArray,
    pub recipe_index: usize,
    pub inv_slots: IndexMap<SlotRange>,
    pub open_notes: Vec<usize>,
}

impl PCSession {
    /// Relies on `PC` to have been provided already to base itself on.
    pub fn new() -> Self {
        PC::untracked(|pc| Self {
            stats: pc.base_stats,
            recipe_index: 0,
            inv_slots: Vec::new().into(),
            open_notes: Vec::new(),
        })
    }

    pub fn with_pc<F, T>(f: F) -> T
    where
        F: FnOnce(&Self, &PC) -> T,
    {
        PC::with(|pc| Self::with(|sesh| f(sesh, pc)))
    }
}

impl RwProvided for PCSession {
    type Item = Self;
}

#[derive(Clone, Copy)]
pub(super) enum SlotRange {
    Single(usize),
    Double(usize),
    Encumbered,
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
            SlotRange::Double(x) => format!("{x} - {}", x + 1).into_view(),
            SlotRange::Encumbered => view! {
                <div class= "fill-red-800 w-4" inner_html=svg::WEIGHT />
            }
            .into_view(),
        }
    }
}
