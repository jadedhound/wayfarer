use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::views::wealth::{wealth_full, wealth_input};

pub fn wealth() -> impl IntoView {
    fn change_wealth<F>(pc: &mut PC, input: RwSignal<u32>, change: F)
    where
        F: Fn(u32, u32) -> u32,
    {
        pc.wealth = change(pc.wealth, input.get());
        input.set(0);
    }
    let pc = PC::expect();
    let input = RwSignal::new(0);
    let curr = PC::slice(|pc| pc.wealth);
    let disabled = create_read_slice(input, |input| input < &1);

    view! {
        <h4 class= "text-center"> "Wealth" </h4>
        <div class= "grid grid-cols-7 gap-x-2">
            <button
                class= "btn bg-red-800 row-span-2 flex-center"
                disabled=disabled
                on:click=move |_| pc.update(|pc| change_wealth(pc, input, |curr, usr| curr.saturating_sub(usr)))
            >
                <div class= "w-5" inner_html=icons::MINUS />
            </button>
            <div class= "col-span-5 flex justify-center py-2 mb-2 bg-zinc-800 rounded">
                { move || wealth_full(curr.get()) }
            </div>
            <button
                class= "btn bg-green-800 row-span-2 flex-center"
                disabled=disabled
                on:click=move |_| pc.update(|pc| change_wealth(pc, input, |curr, usr| curr.saturating_add(usr)))
            >
                <div class= "w-5" inner_html=icons::PLUS />
            </button>
            <div class= "col-span-5">
                { wealth_input(input) }
            </div>
        </div>
    }
}
