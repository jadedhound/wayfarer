use gloo::events::EventListener;
use leptos::*;
use leptos_router::use_location;

use super::attr::MAX_INVENTORY;
use super::shops::Shop;
use super::PC;
use crate::icons;
use crate::pc::pc_stat::StatArray;
use crate::utils::index_map::IndexMap;
use crate::utils::RwProvided;

pub(super) struct PCSession {
    pub pc_id: usize,
    pub stats: StatArray,
    pub inv_slots: IndexMap<SlotRange>,
    pub max_inv: usize,
    pub open_notes: Vec<usize>,
    pub is_enumbered: bool,
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
            inv_slots: Vec::new().into(),
            open_notes: Vec::new(),
            max_inv: MAX_INVENTORY,
            is_enumbered: false,
            revealer_listen: None,
            active_shop: Shop::default(),
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
                <div class= "fill-red-500 w-4" inner_html=icons::WEIGHT />
            }
            .into_view(),
        }
    }
}
