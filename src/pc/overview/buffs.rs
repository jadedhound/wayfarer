use leptos::*;

use crate::items::buffs::{Buff, FeatOrStat};
use crate::pc::{session, PC};
use crate::svg;
use crate::utils::rw_context;
use crate::views::revealer::{Revealer, RevealerScreen};

#[component]
pub(super) fn BuffListview(cx: Scope) -> impl IntoView {
    let buff_list = move || {
        rw_context::<PC>(cx).with(|pc| {
            pc.conditions
                .iter()
                .enumerate()
                .map(|(i, buff)| view! { cx, <BuffView i buff /> })
                .collect_view(cx)
        })
    };

    view! { cx,
        <RevealerScreen />
        <h5 class= "text-center"> "BUFFS AND DEBUFFS" </h5>
        <div class= "flex gap-2 overflow-x-auto h-28 pb-3">
            <button
                class= "rounded border-2 border-zinc-700 h-full w-20 shrink-0 flex-centered"
                on:click=move |_| {}
                disabled
            >
                <div class= "stroke-zinc-700 w-12" inner_html=svg::PLUS />
            </button>
            { buff_list }
        </div>
    }
}

#[component]
fn BuffView<'a>(cx: Scope, i: usize, buff: &'a Buff) -> impl IntoView {
    let Buff {
        name,
        duration,
        effect,
    } = buff;
    let effect = match effect {
        FeatOrStat::Feat(x) => x.effect.clone(),
        FeatOrStat::Stat(x) => x.string_iter().fold(String::new(), |mut acc, e| {
            acc.push_str(&e);
            acc
        }),
    };
    let hidden = move || Revealer::state(cx, i as u32);
    view! { cx,
        <div class= "relative">
            <button
                on:click=move |_| { Revealer::open(cx, i as u32) }
                class= "flex font-sans shrink-0 aspect-video h-full w-full"
                hidden=hidden
            >
                <div class= "w-6 bg-sky-800 flex-centered rounded-l h-full">
                    { *duration }
                </div>
                <div class= "flex-centered flex-col rounded-r bg-zinc-800 px-1 h-full w-full">
                    <div class= "text-center"> { name.to_uppercase() } </div>
                    <div> { effect } </div>
                </div>
            </button>
            <button
                on:click=move |_| {
                    rw_context::<PC>(cx).with(|pc| {
                        session::rm_buff(cx, &pc.conditions[i]);
                    });
                    rw_context::<PC>(cx).update(|pc| {
                        pc.conditions.remove(i);
                    });
                    Revealer::dismiss(cx)
                }
                class= "inset-0 bg-red-700 rounded absolute px-2 z-50"
                hidden=move || !hidden()
            >
                <div class= "flex-centered gap-2">
                    <div class= "svg w-6" inner_html=svg::TRASH />
                    <div> { format!("Remove {name}?") } </div>
                </div>
            </button>
        </div>
    }
}
