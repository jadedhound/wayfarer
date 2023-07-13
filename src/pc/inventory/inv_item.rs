use leptos::*;

use crate::items::{quality_str, Item};
use crate::utils::capitalise;

#[component]
pub fn InvItem(cx: Scope, item: Item) -> impl IntoView {
    let q_item = item.clone();
    let name = move || capitalise(&item.name());
    let quality = move || {
        match &q_item {
            Item::Held(a) => Some(a.quality),
            Item::Armour(a) => Some(a.quality),
            Item::Head(_) => Some(3),
            _ => None,
        }
        .map(|q| quality_str(q))
        .unwrap_or_default()
    };

    view! {
        cx,
        <div class= "flex flex-col h-full justify-center px-2 text-start">
            <div class= "truncate">
                { move || name() }
            </div>
            <div class= "italic text-sm">
                { move || quality() }
            </div>
        </div>
    }
}
