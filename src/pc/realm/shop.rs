use leptos::*;
use leptos_router::use_location;
use strum::{Display, EnumIter, FromRepr};

use crate::icons;
use crate::items::ItemRef;
use crate::pc::session::Session;
use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;
use crate::views::modal::{ModalLocation, ModalState};
use crate::views::wealth::maybe_wealth;

mod cart_view;
pub mod delegate;
pub mod list;
mod shop_view;

#[derive(Clone, Copy, EnumIter, Display, FromRepr, Default)]
pub enum Shop {
    #[strum(serialize = "Adventuring Supplies")]
    Adventurer,
    #[default]
    Alchemist,
    #[strum(serialize = "Arcane Forge")]
    Arcane,
    Armoursmith,
    #[strum(serialize = "Hallowed Ground")]
    Divine,
    Fletcher,
    #[strum(serialize = "Illicit Goods")]
    Illicit,
    Weaponsmith,
}

impl Shop {
    pub fn items(&self) -> &[&'static ItemRef] {
        use crate::items::{
            adventure, alchemist, arcane, armoursmith, divine, fletcher, illicit_goods, weaponsmith,
        };
        match self {
            Shop::Adventurer => &adventure::ITEMS,
            Shop::Alchemist => &alchemist::ITEMS,
            Shop::Arcane => &arcane::ITEMS,
            Shop::Armoursmith => &armoursmith::ITEMS,
            Shop::Divine => &divine::ITEMS,
            Shop::Weaponsmith => &weaponsmith::ITEMS,
            Shop::Fletcher => &fletcher::ITEMS,
            Shop::Illicit => &illicit_goods::ITEMS,
        }
    }

    /// If certain crafting requirements aren't met, the crafting area
    /// cannot be used by the PC.
    pub fn cannot_use(&self, sesh: &Session) -> bool {
        match self {
            Shop::Arcane => sesh.cast_arcane < 1,
            Shop::Divine => sesh.cast_divine < 1,
            _ => false,
        }
    }

    /// Flavourful description of the shop.
    pub fn desc(&self) -> &'static str {
        match self {
            Shop::Adventurer => "The walls are covered with gear of all kinds, some of them not so new.",
            Shop::Alchemist => "Liquids, powders and gases fill containers of all sizes, best step carefully around these shelves.",
            Shop::Arcane => "Components, mundane to most but not you, for you have gazed beyond the veil and see them for what they truly are.",
            Shop::Armoursmith => "The sour smell of sweat, a tide of heat and the rythmic beats of a hammer assault your senses.",
            Shop::Divine => "You step on holy ground, the gods are likely to listen to your requests for aid.",
            Shop::Weaponsmith => "You gaze at the weapons, each one more deadly than the last; you are sure they would do well in your hands.",
            Shop::Fletcher => "The arrows you see alarm and fanscinate you, perhaps you should buy one, just for curiousity's sake",
            Shop::Illicit => "You can't help but be nervous in this place, best to make your purchases quickly and leave without attracting attention",
        }
    }
}

#[derive(Clone)]
struct State {
    shop: Shop,
    is_cart: bool,
    pub cart: IndexMap<&'static ItemRef>,
    price: u32,
    weight: usize,
    item_details: &'static ItemRef,
    search_results: Option<Vec<&'static ItemRef>>,
}

impl RwUtils for State {}

impl Default for State {
    fn default() -> Self {
        let shop = {
            let path = use_location().pathname.get_untracked();
            path.split('/')
                .last()
                .and_then(|last_word| {
                    let i = last_word.parse::<usize>().ok()?;
                    Shop::from_repr(i)
                })
                .unwrap_or_default()
        };
        Self {
            shop,
            is_cart: false,
            cart: Default::default(),
            price: 0,
            weight: 0,
            item_details: Default::default(),
            search_results: None,
        }
    }
}

fn shop_item_view<F>(item_ref: &'static ItemRef, on_click: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let state = State::expect();
    let open_details = move |_| {
        ModalState::show(ModalLocation::ShopItemDetails);
        state.update(|x| x.item_details = item_ref);
    };

    view! {
        <div class= "p-2 flex gap-2">
            <button
                class= "w-4 stroke-sky-500"
                on:click=open_details
                hidden=move || item_ref.props.is_empty()
                inner_html=icons::INFO
            />
            <button
                class= "capitalise w-12 grow text-left"
                on:click= move |_| on_click()
            >
                { item_ref.name }
            </button>
            { maybe_wealth(item_ref.price()) }
        </div>
    }
}
