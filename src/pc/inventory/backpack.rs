use leptos::*;

use crate::items::buffs::Buff;
use crate::items::Item;
use crate::pc::inventory::stack_btn::stack_btn;
use crate::pc::pc_stat::PCStat;
use crate::pc::session::{PCSession, SlotRange};
use crate::pc::PC;
use crate::svg;
use crate::utils::{expect_rw, some_if, RwProvided};
use crate::views::revealer::Revealer;

#[derive(Clone, Copy)]
struct DeleteState(Option<usize>);

pub fn backpack() -> impl IntoView {
    let delete = create_rw_signal( DeleteState(None));
    provide_context( delete);

    view! { 
        { list_view() }
        <div
            class= "psuedo fixed h-cover w-full z-10 top-0 right-0"
            hidden=move || delete.with(|d| d.0.is_none())
            on:click=move |_| delete.update(|d| d.0 = None)
        />
    }
}

fn list_view() -> impl IntoView {
    let id_list = move || {
        PC::with( |pc| {
            pc.inventory.clone_iter().collect::<Vec<(usize, Item)>>()
        })
    };

    view! { 
        <div class= "flex flex-col shaded-table">
            <For
                each=id_list
                key=|(id, _)| *id
                view=move | (id, item)| item_view( id, item)
            />
            { empty_slots() }
        </div>
    }
}

/// Shows empty slots for a PC.
fn empty_slots() -> impl IntoView {
    move || {
        PCSession::with( |sesh| {
            let max = sesh.stats.get(PCStat::Inventory) as usize;
            let curr = sesh
                .inv_slots
                .values()
                .last()
                .map(|x| match x {
                    SlotRange::Single(x) => *x,
                    SlotRange::Double(x) => *x + 1,
                    SlotRange::Encumbered => max,
                })
                .unwrap_or(max);
            some_if(curr <= max).map(|_| {
                (curr + 1..max + 1)
                    .map(|i| {
                        view! { 
                            <div class= "flex">
                                <div class= "w-12 flex-centered"> { i } </div>
                                <div class= "psuedo h-20 w-12 grow" />
                            </div>
                        }
                    })
                    .collect_view()
            })
        })
    }
}

/// Renders the item with the `id` given.
fn item_view( id: usize, item: Item) -> impl IntoView {
    let range = move || {
        PCSession::with( |sesh| match sesh.inv_slots.get(id) {
            Some(x) => *x,
            None => SlotRange::Single(0),
        })
    };
    let stacks = some_if(item.stacks.is_some()).map(|_| stack_btn(id, true));

    view! {
        <div class= "flex gap-2">
            <div class= "w-12 flex-centered"> { range } </div>
            <div class= "py-2 w-12 grow"> { item.into_view() } </div>
            { stacks }
            { more_btn( id) }
        </div>
    }
}

fn more_btn( id: usize) -> impl IntoView {
    let pc = expect_rw::<PC>();
    let show_menu = move || Revealer::state( 'm', id);
    let delete = move || {
        Revealer::dismiss();
        pc.update(|pc| {
            pc.inventory.remove(id);
        });
    };
    let to_quick = move || {
        Revealer::dismiss();
        pc.update(|pc| {
            pc.inventory.remove(id).and_then(|item| {
                let i = pc.quick_access.iter().position(|x| x.is_none())?;
                pc.quick_access[i] = Some(item);
                Some(())
            });
        });
    };
    let add_buff = move |mut buff: Buff| {
        Revealer::dismiss();
        PC::update( |pc| {
            buff.duration.set(&pc.turns);
            pc.buffs.add(buff);
            pc.inventory.remove(id);
        })
    };
    let popup = move || {
        const BTN_CSS: &str = "w-24 py-2 disabled:text-zinc-500";
        let to_quick_disabled = move || {
            pc.with(|pc| {
                let curr = pc.quick_access.iter().flatten().count();
                curr >= pc.quick_access.len()
            })
        };
        let use_item = move || {
            let maybe_buff = PC::with( |pc| {
                let item = pc.inventory.get(id).unwrap();
                item.spec.as_buff().cloned()
            });
            maybe_buff.map(|b| {
                view! {
                    <button
                        class=BTN_CSS
                        on:click=move |_| add_buff(b.clone())
                    >
                        "USE"
                    </button>
                }
            })
        };
        some_if(show_menu()).map(|_| {
            view! { 
                <div class= "rounded bg-zinc-800 border-2 border-zinc-600 absolute font-sans z-40 flex flex-col divide-y divide-dashed px-2 translate-y-1 -translate-x-[5.5rem]">
                    <button
                        class=BTN_CSS
                        on:click=move |_| delete()
                    >
                        "DELETE"
                    </button>
                    <button
                        class=BTN_CSS
                        on:click=move |_| to_quick()
                        disabled=to_quick_disabled
                    >
                        "QUICK ACCESS"
                    </button>
                    { use_item }
                </div>
            }
        })
    };

    view! { 
        <button
            class= "flex-centered w-8"
            on:click=move |_| Revealer::open( 'm', id)
        >
            <div class= "relative">
                <div class= "w-6 svg" inner_html=svg::VERT_ELLIPS />
                { popup }
            </div>
        </button>
    }
}
