use std::cmp;

use const_format::formatcp;
use leptos::ev::Event;
use leptos::*;

use crate::icons;
use crate::utils::some_if;

const STYLES: [&str; 3] = ["fill-yellow-500", "fill-stone-300", "fill-orange-800"];

pub fn funds(sup: u32) -> impl IntoView {
    fund_view(coinage_and_colour(sup))
}

pub fn short_funds<F>(sup: F) -> impl IntoView
where
    F: Fn() -> u32 + 'static,
{
    let sup = sup();
    some_if(sup > 0).map(|_| {
        let non_zero = coinage_and_colour(sup).filter(|x| x.0 != 0);
        fund_view(non_zero)
    })
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

fn fund_view<C>(coins: C) -> impl IntoView
where
    C: Iterator<Item = (u32, &'static str)>,
{
    let coins = coins
        .into_iter()
        .map(|(num, colour)| {
            view! {
                <div> { num } </div>
                <div class=format!("{colour} w-4 translate-y-1") inner_html=icons::CIRCLE />
            }
        })
        .collect_view();
    view! { <div class= "flex gap-1"> { coins } </div> }
}

fn coinage_and_colour(total: u32) -> impl Iterator<Item = (u32, &'static str)> {
    split_into_coinage(total).into_iter().zip(STYLES)
}

fn split_into_coinage(mut total: u32) -> [u32; 3] {
    let cp = total % 10;
    total /= 10;
    let sp = total % 100;
    total /= 100;
    [total, sp, cp]
}
