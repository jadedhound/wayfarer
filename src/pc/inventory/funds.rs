use leptos::*;

use crate::pc::PC;
use crate::svg;
use crate::utils::rw_context;
use crate::views::Funds;

#[component]
pub fn EditableFunds(cx: Scope) -> impl IntoView {
    let pc = rw_context::<PC>(cx);
    let usr_fund = create_rw_signal(cx, 0);
    let chg_fund = move |m: i64| {
        pc.update(|pc| {
            let new_fund = pc.supply as i64 + (m * usr_fund.get());
            pc.supply = u32::try_from(new_fund).unwrap_or(0);
            usr_fund.set(0);
        })
    };
    view! {
        cx,
        <div class= "flex gap-2">
            <button
                class= "rounded p-2 bg-zinc-800"
                on:click=move |_| chg_fund(1)
            >
                <div class= "w-8 stroke-green-500" inner_html=svg::PLUS />
            </button>
            <div class= "flex flex-col grow w-12 text-center">
                <Funds sup=move || pc.with(|pc| pc.supply) />
                <div class= "relative mt-2">
                    <div class= "h-10 rounded flex-centered bg-zinc-800">
                        <Funds sup=move || usr_fund.with(|f| *f as u32) />
                    </div>
                    <input
                        class= "h-10 inset-0 opacity-0 absolute caret-transparent outline-none text-left rounded bg-zinc-800 border-2 border-sky-800"
                        type= "number"
                        on:focus=move |_| usr_fund.set(0)
                        on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<i64>().unwrap_or(0);
                            usr_fund.set(val)
                        }
                        prop:value=move || usr_fund.get()
                    />
                </div>
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
