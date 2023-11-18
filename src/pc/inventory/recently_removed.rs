use leptos::*;

use crate::icons;
use crate::items::Item;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::views::modal::{ModalCenter, ModalLocation, ModalState};

pub fn button() -> impl IntoView {
    let on_click = move |_| ModalState::show(ModalLocation::RecentlyRemoved);

    view! {
        <button
            class= "btn bg-indigo-900"
            on:click=on_click
        >
            "RECENTLY REMOVED"
        </button>
    }
}

pub fn modal() -> impl IntoView {
    let removed = PC::slice(|pc| pc.recently_removed.clone());
    let item_list = move || {
        removed
            .get()
            .iter()
            .rev()
            .cloned()
            .map(add_back_item)
            .collect_view()
    };
    view! {
        <ModalCenter location=ModalLocation::RecentlyRemoved>
            <h4 class= "text-center"> "Recently Removed" </h4>
            <div class= "shaded-table-surface flex flex-col gap-2 empty:hidden">
                { item_list }
            </div>
            <div class= "italic text-center" hidden=move || !removed.get().is_empty()>
                "Recently deleted or used items will be listed here."
            </div>
        </ModalCenter>
    }
}

fn add_back_item(item: Item) -> impl IntoView {
    let pc = PC::expect();
    let item_view = item.into_view();
    let add_back = move |_| {
        pc.update(|pc| {
            let mut item = item.clone();
            if let Some(counter) = item.find_mut_counter() {
                counter.curr = counter.max
            }
            pc.recently_removed.remove_where(|removed| removed == &item);
            pc.inventory.add(item);
        })
    };

    view! {
        <button
            class= "flex items-center gap-2 p-2"
            on:click=add_back
        >
            <div class= "w-12 grow">
                { item_view }
            </div>
            <div class= "w-6" inner_html=icons::PLUS />
        </button>
    }
}
