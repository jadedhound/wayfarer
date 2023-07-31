use leptos::*;

use crate::items::item_specs::ItemSpec;
use crate::items::Item;
use crate::pc::{session, PC};
use crate::utils::rw_context;
use crate::views::revealer::{Revealer, RevealerScreen};

#[component]
pub(super) fn BuffItem<'a>(cx: Scope, id: u32, item: &'a Item) -> impl IntoView {
    let name = item.name.clone();
    let hide_btn = move || Revealer::state(cx, id);

    view! {
        cx,
        <RevealerScreen />
        <div class= "col-span-7 relative">
            <button
                class= "bg-zinc-700 py-1 rounded-r absolute z-50 inset-0"
                on:click=move |_| {
                    use_item(cx, id);
                    Revealer::dismiss(cx)
                }
                hidden= move || !hide_btn()
            >
                { format!("Use {}?", name) }
            </button>
            <button
                class= "bg-zinc-800 py-1 w-full rounded-r"
                on:click=move |_| { Revealer::open(cx, id) }
            >
                { item.into_view(cx) }
            </button>
        </div>
    }
}

fn use_item(cx: Scope, id: u32) {
    rw_context::<PC>(cx).update(|pc| {
        let item = pc.inventory.remove(id as usize);
        if let ItemSpec::Buff(x) = item.spec {
            session::add_buff(cx, &x);
            pc.conditions.push(x);
        }
    })
}
