use leptos::*;

use crate::items::Item;
use crate::pc::session::PCSession;
use crate::pc::PC;
use crate::utils::rw_context;
use crate::views::revealer::Revealer;

#[component]
pub(super) fn BuffItem<'a>(cx: Scope, id: usize, item: &'a Item) -> impl IntoView {
    let name = item.name.clone();
    let hide_btn = move || Revealer::state(cx, 'b', &id);

    view! {
        cx,
        <div class= "col-span-7 relative">
            <button
                class= "bg-zinc-700 py-1 rounded-r absolute z-50 inset-0"
                on:click=move |_| {
                    use_item(cx, &id);
                    Revealer::dismiss(cx)
                }
                hidden= move || !hide_btn()
            >
                { format!("Use {}?", name) }
            </button>
            <button
                class= "bg-zinc-800 py-1 w-full rounded-r"
                on:click=move |_| { Revealer::open(cx, 'b', &id) }
            >
                { item.into_view(cx) }
            </button>
        </div>
    }
}

fn use_item(cx: Scope, id: &usize) {
    rw_context::<PC>(cx).update(|pc| {
        pc.inventory.remove(id).and_then(|x| {
            let buff = x.spec.as_buff()?;
            rw_context::<PCSession>(cx).update(|sesh| sesh.add_buff(buff));
            pc.buffs.push(buff.clone());
            Some(())
        });
    })
}
