use components::Components;
use leptos::*;
use strum::Display;

use crate::items::craft::{craft, Artisan, Experience};
use crate::items::Item;
use crate::pc::{format_funds, PC};
use crate::utils::{read_context, rw_context, StrPlus};
use crate::views::InvItem;

mod components;
mod supply;

struct CraftState {
    artisan: Artisan,
    exp: Experience,
    item: Option<usize>,
    craft: Option<Item>,
    sup_cost: u32,
    base_dc: u32,
}

impl CraftState {
    fn new() -> Self {
        Self {
            artisan: Artisan::Enchanter,
            exp: Experience::Novice,
            item: None,
            craft: None,
            sup_cost: 0,
            base_dc: 20,
        }
    }

    fn chg_item(&mut self, cx: Scope, i: usize) -> Option<()> {
        let item = read_context::<PC>(cx).with(|pc| pc.inventory.get(i).cloned())?;
        let craft = craft(self.artisan, &item);
        self.sup_cost = (craft.price() - item.price()) / 2;
        self.base_dc = 20 + (item.quality() * 5) as u32;
        log::info!(
            "craft: {}, item: {}, sup_cost: {}, DC: {}",
            craft.price(),
            item.price(),
            self.sup_cost,
            self.base_dc
        );
        self.item = Some(i);
        self.craft = Some(craft);
        Some(())
    }
}

#[component]
pub fn Crafting(cx: Scope) -> impl IntoView {
    let _pc = read_context::<PC>(cx);
    let state = create_rw_signal(cx, CraftState::new());
    provide_context(cx, state);

    view! {
        cx,
        <div class= "flex flex-col px-2 gap-2">
        <h3 class= "border-b border-amber-600 text-center"> "Craft Item" </h3>
        <Components />
        <Craft />
        <button class= "bg-amber-800 rounded h-12 mt-8">
            "Add to Queue"
        </button>
        <h3 class= "border-b border-amber-600 text-center mt-4"> "Queue" </h3>
        <Queue />
        </div>
    }
}

#[component]
fn Craft(cx: Scope) -> impl IntoView {
    let craft = move || rw_context::<CraftState>(cx).with(|s| s.craft.clone());
    view! {
        cx,
        <div class= "border-2 border-amber-800 rounded p-2 text-center">
            { move || match craft() {
                Some(item) => view!{ cx, <InvItem item=item /> }.into_view(cx),
                None => view!{ cx, }.into_view(cx)
            }}
        </div>
    }
}

#[component]
fn Queue(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div />
    }
}
