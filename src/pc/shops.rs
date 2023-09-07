use leptos::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

use crate::items;
use crate::pc::session::PCSession;
use crate::utils::RwProvided;
use crate::views::revealer::Revealer;

#[derive(Serialize, Deserialize, Clone, Copy, EnumIter, Display, Default)]
pub enum Shop {
    #[default]
    Alchemist,
    #[strum(serialize = "Arcane Workshop")]
    Arcane,
    Blacksmith,
    #[strum(serialize = "Adventuring Supplies")]
    Adventurer,
    #[strum(serialize = "Holy Ground")]
    Holy,
}

pub fn shop() -> impl IntoView {
    let shop = PCSession::with(|sesh| sesh.active_shop);

    view! {
        <h3 class= "text-center"> { shop.to_string() } </h3>
        { item_list(shop) }
        <h3 class= "text-center"> { "Cart" } </h3>
        { purchase_btn }
    }
}

fn item_list(shop: Shop) -> impl IntoView {
    let items = match shop {
        Shop::Alchemist => [items::alchemy::t1::ALL],
        Shop::Arcane => todo!(),
        Shop::Blacksmith => todo!(),
        Shop::Adventurer => todo!(),
        Shop::Holy => todo!(),
    };

    view! {}
}

fn purchase_btn() -> impl IntoView {
    view! {
        <div class= "relative">
            <button
                class= "btn bg-surface p-2 w-full"
                on:click=move |_| Revealer::show('p', 0)
            >
                "PURCHASE"
            </button>
            <div hidden=move || Revealer::hidden('p', 0)>
                <button
                    class= "absolute top-0 btn bg-blue-800 h-full w-full"
                    on:click=move |_| {}
                >
                    "CONFIRM"
                </button>
            </div>
        </div>
    }
}
