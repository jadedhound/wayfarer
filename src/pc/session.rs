use gloo::events::EventListener;
use leptos::*;

use crate::icons;
use crate::pc::PCStatArray;
use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;

#[derive(Default)]
pub(super) struct Session {
    pub stats: PCStatArray,
    pub max_inv: usize,
    pub inv_slots: IndexMap<SlotRange>,
    pub empty_inv_slots: usize,
    pub is_encumbered: bool,
    pub open_notes: Vec<usize>,
    pub cast_divine: u8,
    pub cast_arcane: u8,
    pub revealer_listen: Option<EventListener>,
}

impl RwUtils for Session {
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
            SlotRange::Double(x) => format!("{} - {x}", x - 1).into_view(),
            SlotRange::Encumbered => view! {
                <div class= "fill-red-500 w-4" inner_html=icons::WEIGHT />
            }
            .into_view(),
        }
    }
}
