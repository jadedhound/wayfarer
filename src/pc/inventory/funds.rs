use leptos::*;

use crate::pc::PC;
use crate::svg;
use crate::utils::expect_rw;
use crate::views::funds::funds;

pub(super) fn editable_funds() -> impl IntoView {
    let pc = expect_rw::<PC>();
    let usr_fund = create_rw_signal(0);
    let chg_fund = move |m: i64| {
        pc.update(|pc| {
            let new_fund = pc.funds as i64 + (m * usr_fund.get() as i64);
            pc.funds = u32::try_from(new_fund).unwrap_or(0);
            usr_fund.set(0);
        })
    };

    view! {

        <div class= "flex gap-2">
            <button
                class= "rounded p-2 bg-zinc-800"
                on:click=move |_| chg_fund(1)
            >
                <div class= "w-8 stroke-green-500" inner_html=svg::PLUS />
            </button>
            <div class= "flex flex-col grow w-12 text-center">
                { move || funds(pc.with(|pc| pc.funds)) }
                { fund_formatted_input( usr_fund) }
            </div>
            <button
                class= "rounded p-2 bg-zinc-800"
                on:click=move |_| chg_fund(-1)
            >
                <div class= "w-8 stroke-red-500" inner_html=svg::MINUS/>
            </button>
        </div>
    }
}

pub(super) fn fund_formatted_input(rw_fund: RwSignal<u32>) -> impl IntoView {
    let parse_input = move |x: String| {
        if let Ok(num) = x.parse::<u32>() {
            if num > 1000000 {
                999999
            } else {
                num
            }
        } else {
            0
        }
    };

    view! {
        <div class= "relative mt-2">
            <div class= "h-10 rounded flex-centered bg-zinc-800">
                { move || funds(rw_fund.get()) }
            </div>
            <input
                class= "h-10 inset-0 opacity-0 absolute caret-transparent outline-none text-left rounded bg-zinc-800 border-2 border-sky-800"
                type= "number"
                on:focus=move |_| rw_fund.set(0)
                on:input=move |ev| rw_fund.set(parse_input(event_target_value(&ev)))
                prop:value=move || rw_fund.get()
            />
        </div>
    }
}
