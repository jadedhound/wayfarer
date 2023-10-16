use std::cmp;

use const_format::formatcp;
use leptos::ev::Event;
use leptos::*;

use crate::icons;
use crate::utils::some_if;

const STYLES: [&str; 3] = ["fill-yellow-500", "fill-stone-300", "fill-orange-800"];

pub fn funds(cp: u32) -> impl IntoView {
    let coins = split_into_coinage(cp)
        .into_iter()
        .zip(STYLES)
        .map(single_coin)
        .collect_view();
    fund_wrapper(coins)
}

pub fn maybe_funds(cp: u32) -> Option<View> {
    some_if(cp > 0).map(|_| {
        let coins = split_into_coinage(cp)
            .into_iter()
            .zip(STYLES)
            .filter(|x| x.0 != 0)
            .map(single_coin)
            .collect_view();
        fund_wrapper(coins).into_view()
    })
}

pub fn short_funds(cp: u32) -> impl IntoView {
    maybe_funds(cp).unwrap_or(fund_wrapper(single_coin((0, STYLES[2])).into_view()).into_view())
}

pub fn fund_input(rw_fund: RwSignal<u32>) -> impl IntoView {
    let into_num = move |ev: Event, max: u32| {
        event_target_value(&ev)
            .parse::<u32>()
            .map(|num| cmp::min(num, max))
            .unwrap_or(0)
    };
    let change_fund = move |coin: usize, new: u32| {
        let mut coinage = split_into_coinage(rw_fund.get());
        coinage[coin] = new;
        rw_fund.set(coinage[0] * 1000 + coinage[1] * 10 + coinage[2])
    };

    view! {
        <div class= "flex items-center gap-1">
            <input
                class= "input w-10 grow text-center"
                type= "number"
                maxlength= "3"
                on:input=move |ev| change_fund(0, into_num(ev, 999))
                prop:value=move || rw_fund.get() / 1000
            />
            <div class=formatcp!("w-4 {}", STYLES[0]) inner_html=icons::CIRCLE />
            <input
                class= "input w-8 grow text-center"
                type= "number"
                on:input=move |ev| change_fund(1, into_num(ev, 99))
                prop:value=move || (rw_fund.get() / 10) % 100
            />
            <div class=formatcp!("w-4 {}", STYLES[1]) inner_html=icons::CIRCLE />
            <input
                class= "input w-4 grow text-center"
                type= "number"
                on:input=move |ev| change_fund(2, into_num(ev, 9))
                prop:value=move || rw_fund.get() % 10
            />
            <div class=formatcp!("w-4 {}", STYLES[2]) inner_html=icons::CIRCLE />
        </div>
    }
}

fn single_coin((num, colour): (u32, &str)) -> impl IntoView {
    view! {
        <div> { num } </div>
        <div class=format!("{colour} w-4 translate-y-1") inner_html=icons::CIRCLE />
    }
}

fn fund_wrapper(coins: View) -> impl IntoView {
    view! { <div class= "flex gap-1"> { coins } </div> }
}

fn split_into_coinage(mut total: u32) -> [u32; 3] {
    let cp = total % 10;
    total /= 10;
    let sp = total % 100;
    total /= 100;
    [total, sp, cp]
}
