use leptos::*;

use crate::items::Item;
use crate::pc::PC;
use crate::utils::{read_context, StrOps, VecStrOps};
use crate::views::SupplyAsGold;

#[component]
pub fn InvItem(cx: Scope, item: Item) -> impl IntoView {
    let name = item.name().capitalise();
    let quality = match &item {
        Item::Held(a) => Some(a.quality_str()),
        Item::Armour(a) => Some(a.quality_str()),
        Item::Head(_) => Some("Fabulous"),
        _ => None,
    }
    .map(|s| s.to_string());
    let stat_scaling = item.as_held().map(|stat| stat.base.scale_by().to_string());
    let subtext = vec![quality, stat_scaling]
        .flat_concat(", ")
        .unwrap_or("".to_string());
    let price = item.price();
    view! {
        cx,
        <div class= "flex flex-col h-full justify-center px-2 text-start">
            <div class= "truncate">
                { name }
            </div>
            <div class= "italic text-sm">
                { subtext }
            </div>
            <div class= "">
                <SupplyAsGold sup=move || price />
            </div>
        </div>
    }
}

#[component]
pub fn InvItemByIndex(cx: Scope, i: usize) -> impl IntoView {
    let item = move || {
        read_context::<PC>(cx)
            .with(|pc| pc.inventory.get(i).cloned())
            .unwrap_or_default()
    };
    view! {
        cx,
        <InvItem item=item() />
    }
}
