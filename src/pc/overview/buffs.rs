use leptos::*;

use crate::items::buffs::Buff;
use crate::pc::PC;
use crate::svg;
use crate::utils::{some_if, RwProvided};

pub(super) fn buff_listview() -> impl IntoView {
    let buff_list = move || {
        PC::with(|pc| {
            some_if(!pc.buffs.is_empty()).map(|_| {
                let v = pc
                    .buffs
                    .iter()
                    .map(|(id, buff)| buff_view(id, buff))
                    .collect_view();
                view! {
                <div class= "shaded-table flex flex-col gap-2">
                    { v }
                </div>
                    }
            })
        })
    };

    view! {
        <div class= "">
            { add_buff_btn() }
            { buff_list }
        </div>
    }
}

fn buff_view(id: usize, buff: &Buff) -> impl IntoView {
    let effect = buff
        .effect
        .as_ref()
        .map(|x| x.desc.clone())
        .unwrap_or_default();
    let stats = buff.stats.map(|x| x.to_string());
    let del_buff = move || PC::update(|pc| pc.buffs.remove(id));
    let duration = buff.duration;
    let turns_left = move || {
        let x = PC::with(|pc| pc.turns.diff(duration));
        format!("{x} left")
    };

    view! {
        <div class= "flex gap-2">
            <button
                on:click=move |_| del_buff()
                class= "flex-centered px-2"
            >
                <div class= "stroke-red-800 w-4" inner_html=svg::CROSS />
            </button>
            <div class= "flex flex-col px-1 w-12 grow py-2">
                <div class= "title uppercase"> { &buff.name } </div>
                <div> { stats } </div>
                <div> { effect } </div>
                <div> { turns_left } </div>
            </div>
        </div>
    }
}

fn add_buff_btn() -> impl IntoView {
    view! {
        <button
            class= "rounded border-2 border-teal-700 text-teal-500 font-sans w-full py-2 mb-2"
            on:click=move |_| {}
            disabled
        >
            "ADD BUFF/DEBUFF"
        </button>
    }
}
