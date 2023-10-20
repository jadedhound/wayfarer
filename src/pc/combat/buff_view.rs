use leptos::*;

use crate::buffs::{Buff, BuffProp};
use crate::icons;
use crate::pc::PC;
use crate::utils::counter::Counter;
use crate::utils::rw_utils::RwUtils;
use crate::views::delete_confirm::DeleteModal;

pub(super) fn view((id, buff): (usize, &Buff)) -> impl IntoView {
    let show_delete_modal = move |_| DeleteModal::show(id);
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
            (count.max < 2)
                .then(|| just_button(id, count).into_view())
                .or_else(|| Some(input_range(id, count).into_view()))
        })
    };

    view! {
            <div class= "p-2">
                <div class= "flex gap-3">
                    <button on:click=show_delete_modal>
                        <div class= "w-5 fill-red-600" inner_html=icons::TRASH />
                    </button>
                    <div class= "mb-2"> { buff.into_view() } </div>
                </div>
                { use_item }
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
    let pc = PC::expect();
    let onclick = move || {
        pc.update(|pc| {
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
            class=format!("btn py-2 {css} w-full")
            on:click=move |_| onclick()
        >
            { text }
        </button>
    }
}

fn input_range(id: usize, count: Counter) -> impl IntoView {
    let pc = PC::expect();
    let onclick = move || {
        pc.update(|pc| {
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
                class=format!("btn {css} p-2")
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
