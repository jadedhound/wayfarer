use leptos::*;

use crate::pc::inventory::backpack::DeleteState;
use crate::pc::{MAX_CAPACITY, PC};
use crate::svg;
use crate::utils::rw_context;

#[component]
pub(super) fn WeightBtn(cx: Scope, i: u8, i_end: Option<u8>, item_i: usize) -> impl IntoView {
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
            let pc = rw_context::<PC>(cx);
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
            class=move || delete.with(|d| format!("rounded-l flex-centered {}", warn(d)))
            on:click=move |_| {
                click(delete.get())
            }
        >
            { move || delete.with(svg_or_text) }
        </button>
    }
}
