use leptos::*;

use crate::items::Item;
use crate::pc::inventory::search::Search;
use crate::pc::{format_funds, MAX_CAPACITY, PC};
use crate::svg;
use crate::utils::{read_context, rw_context, write_context, StrPlus};
use crate::views::InvItem;

#[derive(Clone)]
struct DeleteState(Option<usize>);

#[component]
pub fn Backpack(cx: Scope) -> impl IntoView {
    let pc = read_context::<PC>(cx);
    let delete = create_rw_signal(cx, DeleteState(None));
    provide_context(cx, delete);

    view! {
        cx,
        <h4 class= "border-b-2 border-sky-900 text-center"> "Backpack" </h4>
        <Funds />
        <div class= "grid grid-cols-8 grid-flow-row gap-2 mt-4">
            { move || pc.with(|pc| backpack_view(cx, pc)) }
        </div>
        <Search />
        <button class= "text-center rounded bg-sky-900 w-full py-1 mt-4">
            "Custom Item"
        </button>
        <div class= "psuedo h-6" />
        <div
            class= "psuedo fixed h-cover w-full z-10 top-0 right-0"
            hidden=move || delete.with(|d| d.0.is_none())
            on:click=move |_| delete.update(|d| d.0 = None)
        />
    }
}

#[component]
pub fn Funds(cx: Scope) -> impl IntoView {
    let pc = read_context::<PC>(cx);
    let pc_write = write_context::<PC>(cx);
    let usr_fund = create_rw_signal(cx, 0);
    let chg_fund = move |m: i64| {
        pc_write.update(|pc| {
            let new_fund = pc.supply as i64 + (m * usr_fund.get());
            pc.supply = u32::try_from(new_fund).unwrap_or(0);
            usr_fund.set(0);
        })
    };
    view! {
        cx,
        <div class= "flex gap-2">
            <button
                class= "rounded p-2 bg-zinc-800"
                on:click=move |_| chg_fund(1)
            >
                <div class= "w-8 stroke-green-500" inner_html=svg::PLUS />
            </button>
            <div class= "flex flex-col grow w-12 text-center">
                <div> { move || pc.with(|pc| format_funds(pc.supply)) } </div>
                <div class= "relative mt-2">
                    <div class= "h-10 border-2 border-sky-900 rounded flex-centered bg-zinc-800">
                        { move || usr_fund.with(|f| format_funds(*f as u32)) }
                    </div>
                    <input
                        class= "h-10 top-0 left-0 w-full opacity-0 absolute outline-none text-left rounded bg-zinc-800 border-2 border-sky-800"
                        type= "number"
                        on:focus=move |_| usr_fund.set(0)
                        on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<i64>().unwrap_or(0);
                            usr_fund.set(val)
                        }
                        prop:value=move || usr_fund.get()
                    />
                </div>
            </div>
            <button
                class= "rounded p-2 bg-zinc-800"
                on:click=move |_| chg_fund(-1)
            >
                <div class= "w-8 stroke-red-500" inner_html=svg::MINUS/>
            </button>
        </div>
    }
}

fn backpack_view(cx: Scope, pc: &PC) -> View {
    let mut counter = 1;
    pc.inventory
        .iter()
        .enumerate()
        .map(|(item_i, item)| {
            let item = item.clone();
            let weight = item.weight();
            let (i, i_end) = if weight > 1 {
                (counter, Some(counter + weight - 1))
            } else {
                (counter, None)
            };
            counter += weight;
            view! {
                cx,
                <InvIndex i i_end item_i/>
                <BackpackItem item />
            }
        })
        .collect_view(cx)
}

#[component]
fn InvIndex(cx: Scope, i: u8, i_end: Option<u8>, item_i: usize) -> impl IntoView {
    let delete = rw_context::<DeleteState>(cx);
    let maybe_delete = |d: &DeleteState, i: usize| d.0.filter(|&inv_i| inv_i == i);
    let is_encumbered = |i| {
        if i > MAX_CAPACITY {
            Some(())
        } else {
            None
        }
    };
    let text = move || match i_end {
        Some(i_end) => view! { cx,
            <div class= "flex flex-col text-center">
                <span> {i} </span>
                <span> {i_end} </span>
            </div>
        }
        .into_view(cx),
        None => i.into_view(cx),
    };
    let svg_or_text = move |d: &DeleteState| match maybe_delete(d, item_i) {
        Some(_) => view! { cx, <div class= "w-6 svg" inner_html=svg::TRASH /> }.into_view(cx),
        None => text(),
    };

    let warn = move |d: &DeleteState| {
        let warn = is_encumbered(i).or_else(|| is_encumbered(i_end?));
        match warn {
            Some(_) => match maybe_delete(d, item_i) {
                Some(_) => "bg-red-900 z-20",
                None => "bg-red-900",
            },
            None => match maybe_delete(d, item_i) {
                Some(_) => "bg-red-900 z-20",
                None => "bg-sky-900",
            },
        }
    };
    let click = move |d: DeleteState| match maybe_delete(&d, item_i) {
        Some(inv_i) => {
            let pc = write_context::<PC>(cx);
            pc.update(|pc| {
                pc.inventory.remove(inv_i);
            });
            delete.update(|d| d.0 = None)
        }
        None => delete.update(|d| d.0 = Some(item_i)),
    };
    view! {
        cx,
        <button
            class=move || delete.with(|d|"rounded flex-centered".plus(warn(d)))
            on:click=move |_| {
                click(delete.get())
            }
        >
            { move || delete.with(svg_or_text) }
        </button>
    }
}

#[component]
fn BackpackItem(cx: Scope, item: Item) -> impl IntoView {
    view! {
        cx,
        <div class= "bg-zinc-800 rounded col-span-7 py-1">
            <InvItem item />
        </div>
    }
}
