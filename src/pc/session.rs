use gloo::events::EventListener;
use leptos::*;
use leptos_router::use_location;

use super::attr::MAX_INVENTORY;
use super::realm::shop::Shop;
use super::PC;
use crate::icons;
use crate::pc::PCStatArray;
use crate::utils::index_map::IndexMap;
use crate::utils::RwProvided;

#[derive(Default)]
pub(super) struct PCSession {
    pub pc_id: usize,
    pub stats: PCStatArray,
    pub max_inv: usize,
    pub inv_slots: IndexMap<SlotRange>,
    pub empty_inv_slots: usize,
    pub is_encumbered: bool,
    pub open_notes: Vec<usize>,
    pub cast_divine: u8,
    pub cast_arcane: u8,
    pub revealer_listen: Option<EventListener>,
    pub active_shop: Shop,
}

impl PCSession {
    /// Relies on `PC` to have been provided already to base itself on.
    pub fn new() -> Self {
        let pc_id: usize = use_location().pathname.with_untracked(|loc| {
            loc.chars()
                .skip(4)
                .take_while(|x| x != &'/')
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        });
        PC::untracked(|pc| Self {
            pc_id,
            stats: pc.base_stats,
            max_inv: MAX_INVENTORY,
            ..Default::default()
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
            SlotRange::Double(x) => format!("{} - {x}", x - 1).into_view(),
            SlotRange::Encumbered => view! {
                <div class= "fill-red-500 w-4" inner_html=icons::WEIGHT />
            }
            .into_view(),
        }
    }
}
