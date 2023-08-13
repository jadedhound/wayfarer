use leptos::*;

use crate::items::buffs::Buff;
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::svg;
use crate::utils::rw_context;
use crate::views::revealer::Revealer;

#[component]
pub(super) fn BuffListview(cx: Scope) -> impl IntoView {
    let buff_list = move || {
        rw_context::<PC>(cx).with(|pc| {
            pc.buffs
                .iter()
                .map(|(id, buff)| view! { cx, <BuffView id=*id buff /> })
                .collect_view(cx)
        })
    };

    view! { cx,
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
fn BuffView<'a>(cx: Scope, id: usize, buff: &'a Buff) -> impl IntoView {
    const REV_ORIGIN: char = 'b';
    let Buff {
        name,
        duration,
        effect,
        stats,
    } = buff;
    let effect = effect.as_ref().map(|x| x.desc.clone()).unwrap_or_default();
    let stats = stats.map(|x| x.to_string());
    let hidden = move || Revealer::state(cx, REV_ORIGIN, &id);
    view! { cx,
        <div class= "relative">
            <button
                on:click=move |_| { Revealer::open(cx, REV_ORIGIN, &id) }
                class= "flex font-sans shrink-0 aspect-video h-full w-full"
                hidden=hidden
            >
                <div class= "w-6 bg-sky-800 flex-centered rounded-l h-full">
                    { *duration }
                </div>
                <div class= "flex-centered flex-col rounded-r bg-zinc-800 px-1 h-full w-full">
                    <div class= "text-center"> { name.to_uppercase() } </div>
                    <div> { stats } </div>
                    <div> { effect } </div>
                </div>
            </button>
            <button
                on:click=move |_| {
                    rw_context::<PC>(cx).with(|pc| {
                        rw_context::<PCSession>(cx).update(|sesh| {
                            sesh.rm_buff(pc.buffs.get(&id).unwrap());
                        })
                    });
                    rw_context::<PC>(cx).update(|pc| {
                        pc.buffs.remove(&id);
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
