use leptos::*;

use self::backpack::backpack;
use self::quick_access::quick_access;
use self::search::search_view;
use crate::icons;
use crate::pc::PC;
use crate::utils::RwProvided;
use crate::views::funds::{fund_input, funds};
use crate::views::modal::ModalState;

mod backpack;
mod quick_access;
mod search;
mod stack_btn;

pub fn inventory() -> impl IntoView {
    // TODO: Remove if no popup modal is used in the future.
    let limit_scroll = move || {
        ModalState::get()
            .map(|_| "overflow-y-hidden h-[85vh]")
            .unwrap_or_default()
    };

    view! {
        <div class=move || format!("flex flex-col gap-4 px-2 {}", limit_scroll())>
            <h2> "Inventory" </h2>
            <h5 class= "text-center"> "QUICK ACCESS" </h5>
            { quick_access }
            <h5 class= "text-center"> "BACKPACK" </h5>
            { change_funds }
            { search_view }
            { backpack }
        </div>
    }
}

fn change_funds() -> impl IntoView {
    let usr_fund = create_rw_signal(0);
    let chg_fund = move |modifier: i64| {
        PC::update(|pc| {
            let change_by = modifier * usr_fund.get() as i64;
            let new_fund = pc.funds as i64 + change_by;
            pc.funds = u32::try_from(new_fund).unwrap_or(0);
            usr_fund.set(0);
        })
    };
    let curr_funds = move || PC::with(|pc| funds(pc.funds));

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
                { fund_input(usr_fund) }
            </div>
        </div>
    }
}
