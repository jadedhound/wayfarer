use leptos::*;

use crate::buffs::{Buff, BuffProp};
use crate::icons;
use crate::pc::PC;
use crate::utils::counter::Counter;
use crate::utils::{some_if, RwProvided};
use crate::views::delete_btn::delete_btn;
use crate::views::revealer::Revealer;

pub(super) fn view((id, buff): (usize, &Buff)) -> impl IntoView {
    let counter = PC::slice(move |pc| {
        pc.buffs.get(id).and_then(|x| {
            x.props.iter().find_map(|x| match x {
                BuffProp::Count(count) => Some(*count),
                _ => None,
            })
        })
    });
    let use_item = move || {
        counter.get().and_then(|count| {
            some_if(count.max < 2)
                .map(|_| just_button(id, count).into_view())
                .or_else(|| Some(input_range(id, count).into_view()))
        })
    };
    let del_buff = move || PC::update(|pc| pc.buffs.remove(id));

    view! {
        <div class= "relative">
            <div
                class= "p-2"
                on:contextmenu=move |event| {
                    event.prevent_default();
                    Revealer::show('b', id);
                }
            >
                <div class= "mb-2"> { buff.into_view() } </div>
                { use_item }
            </div>
            { delete_btn('b', id, del_buff) }
        </div>
    }
}

fn change_count(pc: &mut PC, id: usize, amount: usize) {
    let counter = pc.buffs.get_mut(id).and_then(|buff| {
        buff.props.iter_mut().find_map(|x| match x {
            BuffProp::Count(count) => Some(count),
            _ => None,
        })
    });
    if let Some(counter) = counter {
        counter.curr = amount
    }
}

fn just_button(id: usize, count: Counter) -> impl IntoView {
    let onclick = move || {
        PC::update(|pc| {
            if count.curr < 1 {
                change_count(pc, id, count.max)
            } else {
                change_count(pc, id, 0)
            }
        })
    };
    let (text, css) = if count.curr < 1 {
        ("RECHARGE", "bg-zinc-700")
    } else {
        ("USE", "bg-sky-800")
    };

    view! {
        <button
            class=format!("btn {css} w-full")
            on:click=move |_| onclick()
        >
            { text }
        </button>
    }
}

fn input_range(id: usize, count: Counter) -> impl IntoView {
    let onclick = move || {
        PC::update(|pc| {
            if count.curr < 1 {
                change_count(pc, id, count.max)
            } else {
                change_count(pc, id, count.curr - 1)
            }
        })
    };
    let (ico, css) = if count.curr < 1 {
        (icons::REFRESH, "bg-zinc-700")
    } else {
        (icons::BOLT, "bg-sky-800")
    };
    let steps = (1..count.max)
        .map(|_| view! { <div class= "w-1 psuedo bg-sky-950" /> })
        .collect_view();

    view! {
        <div class= "flex gap-1">
            <button
                class=format!("btn {css} px-2")
                on:click=move |_| onclick()
            >
                <div class= "w-6" inner_html=ico />
            </button>
            <div class= "relative w-12 grow">
                <input
                    class= "range sky-bar bg-sky-950 w-full h-full"
                    type= "range"
                    min=0
                    max=count.max
                    value=count.curr
                    step=1
                    disabled=true
                />
                <div class= "absolute w-full flex justify-evenly top-0 h-full">
                    { steps }
                </div>
            </div>
        </div>
    }
}
