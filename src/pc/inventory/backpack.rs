use leptos::*;

use crate::icons;
use crate::pc::inventory::edit_item_modal;
use crate::pc::inventory::stack_btn::stack_btn;
use crate::pc::session::Session;
use crate::pc::PC;
use crate::utils::concat_if;
use crate::utils::rw_utils::RwUtils;
use crate::views::delete_confirm::DeleteModal;
use crate::views::funds::maybe_funds;
use crate::views::modal::ModalState;
use crate::views::revealer::Revealer;

pub fn backpack() -> impl IntoView {
    let pc = PC::expect();
    let id_list = move || pc.with(|pc| pc.inventory.keys().collect::<Vec<_>>());
    DeleteModal::set_effect(move |id| {
        pc.update(|pc| {
            pc.quick_access.remove_where(|x| *x == id);
            pc.inventory.remove(id);
        })
    });

    view! {
        <div class= "flex flex-col shaded-table">
            <For
                each=id_list
                key=|id| *id
                children=item_view
            />
            { empty_slots }
        </div>
    }
}

/// Shows empty slots for a PC.
fn empty_slots() -> impl IntoView {
    let sesh = Session::expect();
    let empty = move |i| {
        view! {
            <div class= "flex">
                <div class= "w-12 flex-center"> { i } </div>
                <div class= "psuedo h-20 w-12 grow" />
            </div>
        }
    };

    move || {
        sesh.with(|sesh| {
            (sesh.empty_inv_slots > 0).then(|| {
                (sesh.max_inv - sesh.empty_inv_slots + 1..=sesh.max_inv)
                    .map(empty)
                    .collect_view()
            })
        })
    }
}

/// Renders the item with the `id` given.
fn item_view(id: usize) -> impl IntoView {
    let pc = PC::expect();
    let item = move || pc.with(|pc| pc.inventory.get(id).cloned().unwrap_or_default());
    let stacks = move || item().find_counter().map(|_| stack_btn(id));
    let item_view = move || item().into_view();
    let price = move || maybe_funds(item().price());
    view! {
        <div class= "relative">
            <div class= "flex gap-2 w-full items-stretch">
                { slot_by_weight(id) }
                <div class= "py-2 w-12 grow flex flex-col">
                    { item_view }
                    <div class= "flex items-center justify-between flex-wrap">
                        { price }
                        { stacks }
                    </div>
                </div>
                { more_button(id) }
            </div>
        </div>
    }
}

fn slot_by_weight(id: usize) -> impl IntoView {
    let (pc, sesh) = (PC::expect(), Session::expect());
    let is_quick = PC::slice(move |pc| pc.quick_access.iter().any(|x| x == &id));
    let disabled = PC::slice(move |pc| pc.quick_access.is_full() && !is_quick.get_untracked());
    let toggle_quick = move |_| {
        if is_quick.get() {
            pc.update(|pc| {
                pc.quick_access.remove_where(|x| *x == id);
            });
        } else {
            pc.update(|pc| pc.quick_access.push(id));
        }
    };
    let icon_or_range = move || {
        if disabled.get() {
            let range = sesh.with(|sesh| sesh.inv_slots.get(id).copied().unwrap_or_default());
            view! {
                <div class= "text-center">
                    { range }
                </div>
            }
            .into_view()
        } else {
            view! {
                <div class= "w-4" inner_html=icons::STAR />
            }
            .into_view()
        }
    };

    view! {
        <button
            class=concat_if(
                move || is_quick.get(),
                "stroke-yellow-500 fill-transparent w-12 flex-center",
                "fill-yellow-500"
            )
            on:click=toggle_quick
            disabled=disabled
        >
            { icon_or_range }
        </button>
    }
}

fn more_button(id: usize) -> impl IntoView {
    let (pc, edit_state) = (PC::expect(), edit_item_modal::State::expect());
    let show_delete_modal = move |_| {
        Revealer::hide();
        DeleteModal::show(id)
    };
    let show_edit_modal = move |_| {
        Revealer::hide();
        if let Some(item) = pc.with(|pc| pc.inventory.get(id).cloned()) {
            edit_state.update(|state| {
                state.id = id;
                state.item = item;
            });
            ModalState::show(10);
        }
    };
    let menu_hidden = create_memo(move |_| !Revealer::is_shown('m', id));

    view! {
        <div class= "flex-center">
            <div class= "relative">
                <button
                    class= "px-2 relative"
                    on:click=move |_| Revealer::show('m', id)
                >
                    <div class= "w-6" inner_html=icons::ELLIPSES />
                </button>
                <div
                    class= "btn bg-surface flex flex-col px-2 z-40 w-28
                    absolute bottom-0 right-0 translate-y-[6.5rem] -translate-x-2"
                    hidden=menu_hidden
                >
                    <button class= "py-3" on:click=show_edit_modal>
                        "EDIT"
                    </button>
                    <button class= "py-3 text-red-500" on:click=show_delete_modal>
                        "DELETE"
                    </button>
                </div>
            </div>
        </div>
    }
}
