use std::cmp::min;

use leptos::*;

use crate::pc::crafting::CraftState;
use crate::pc::{format_funds, PC};
use crate::utils::{read_context, rw_context};
use crate::views::SupplyAsGold;

#[component]
pub fn Supply(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <CurrSupply />
        <Time/>
        <Difficulty />
        <RemSupply />
    }
}

#[component]
fn CurrSupply(cx: Scope) -> impl IntoView {
    let curr = move || read_context::<PC>(cx).with(|pc| pc.supply);
    view! {
        cx,
        <div class= "col-span-3 pr-2">
            "Current Supply"
        </div>
        <div class= "col-span-2">
            <SupplyAsGold sup=move || curr() />
        </div>
    }
}

#[component]
fn Time(cx: Scope) -> impl IntoView {
    let usr_time = create_rw_signal(cx, 0);
    let state = rw_context::<CraftState>(cx);
    let range = create_memo(cx, move |_| {
        let time = state.with(|s| s.sup_cost / s.exp.to_time());
        let scale = (time as f32 * 0.05).ceil() as u32;
        let max = time + (scale * 5);
        (max, scale)
    });
    let curr_time = move || {
        let (max, scale) = range.get();
        max - (scale * usr_time.get())
    };
    let cost = move || {
        let base = state.with(|s| s.sup_cost / 2);
        let added = usr_time.get() * state.with(|s| s.exp.to_time());
        base + added
    };

    view! {
        cx,
        <div class= "col-span-3 p-1">
            <span> { move || curr_time() } </span>
            <span> " days required" </span>
        </div>
        <div class= "col-span-2 row-span-2">
            <SupplyAsGold sup=move || cost() />
        </div>
        <div class= "col-span-3 flex-centered">
            <input
                class= "w-full"
                type= "range"
                min=0
                max=9
                on:input=move |ev| usr_time.set(event_target_value(&ev).parse::<u32>().unwrap())
                prop:value=move || usr_time.get()
            />
        </div>
    }
}

#[component]
fn Difficulty(cx: Scope) -> impl IntoView {
    let usr_dc = create_rw_signal(cx, 0);
    let state = rw_context::<CraftState>(cx);
    let dc = create_memo(cx, move |_| state.with(|s| s.base_dc - s.exp.to_dc()));
    let step_cost = create_memo(cx, move |_| {
        let total = state.with(|s| s.sup_cost) as f64;
        let fraction = (total * 0.05).ceil();
        fraction as u32
    });
    let curr_dc = move || {
        let dc = dc.get() - usr_dc.get();
        if dc > 20 {
            "Impossible".into_view(cx)
        } else if dc < 6 {
            "Guaranteed Success".into_view(cx)
        } else {
            format!("DC {dc}").into_view(cx)
        }
    };
    let cost = move || {
        let base = state.with(|s| s.sup_cost / 2);
        let added = usr_dc.get() * step_cost.get();
        base + added
    };

    view! {
        cx,
        <div class= "col-span-3 p-1">
            { move || curr_dc() }
        </div>
        <div class= "col-span-2 row-span-2">
            <SupplyAsGold sup=move || cost() />
        </div>
        <div class= "col-span-3 flex-centered">
            <input
                class= "w-full"
                type= "range"
                min=0
                max=10
                on:input=move |ev| usr_dc.set(event_target_value(&ev).parse::<u32>().unwrap())
                prop:value=move || usr_dc.get()
            />
        </div>
    }
}

#[component]
fn RemSupply(cx: Scope) -> impl IntoView {
    let curr = move || read_context::<PC>(cx).with(|pc| pc.supply);
    view! {
        cx,
        <div class= "col-span-3 pr-2">
            "Remaining"
        </div>
        <div class= "col-span-2">
            { move || view!{
                cx,
                <SupplyAsGold sup=move || curr() />
            }}
        </div>
    }
}
