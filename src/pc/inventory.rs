use leptos::*;

use self::backpack::backpack;
use self::search::search_view;
use crate::icons;
use crate::pc::inventory::edit_item_modal::edit_item_modal;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::views::funds::{funds, wealth_input};

mod backpack;
mod edit_item_modal;
mod search;
mod stack_btn;

#[component]
pub fn Inventory() -> impl IntoView {
    edit_item_modal::State::provide();

    view! {
        <h4 class= "text-center"> "Wealth" </h4>
        { change_funds }
        <h4 class= "text-center"> "Backpack" </h4>
        { search_view }
        { backpack }
        { edit_item_modal }
    }
}

fn change_funds() -> impl IntoView {
    let pc = PC::expect();
    let input = RwSignal::new(0);
    let chg_fund = move |modifier: i64| {
        pc.update(|pc| {
            let change_by = modifier * input.get() as i64;
            let new_fund = pc.wealth as i64 + change_by;
            pc.wealth = u32::try_from(new_fund).unwrap_or(0);
            input.set(0);
        })
    };
    let curr_funds = move || pc.with(|pc| funds(pc.wealth));

    view! {
        <div class= "grid grid-cols-7 gap-x-2">
            <button
                class= "btn bg-green-800 row-span-2 flex-center"
                on:click=move |_| chg_fund(1)
            >
                <div class= "w-5" inner_html=icons::PLUS />
            </button>
            <div class= "col-span-5 flex justify-center py-2 mb-2 bg-zinc-800 rounded">
                { curr_funds }
            </div>
            <button
                class= "btn bg-red-800 row-span-2 flex-center"
                on:click=move |_| chg_fund(-1)
            >
                <div class= "w-5" inner_html=icons::MINUS />
            </button>
            <div class= "col-span-5">
                { wealth_input(input) }
            </div>
        </div>
    }
}
