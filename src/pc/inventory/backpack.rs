pub mod stack_btn;
pub mod usable;
pub mod weight_btn;

use const_format::concatcp;
use leptos::*;

use crate::items::item_specs::ItemSpec;
use crate::items::Item;
use crate::pc::inventory::backpack::stack_btn::StackBtn;
use crate::pc::inventory::backpack::usable::BuffItem;
use crate::pc::inventory::backpack::weight_btn::WeightBtn;
use crate::pc::PC;
use crate::utils::rw_context;

#[derive(Clone, Copy)]
struct DeleteState(Option<usize>);

#[component]
pub fn Backpack(cx: Scope) -> impl IntoView {
    let pc = rw_context::<PC>(cx);
    let delete = create_rw_signal(cx, DeleteState(None));
    provide_context(cx, delete);

    view! {
        cx,
        <div class= "grid grid-cols-8 grid-flow-row gap-y-2 mt-4">
            { move || pc.with(|pc| backpack_view(cx, pc)) }
        </div>
        <div
            class= "psuedo fixed h-cover w-full z-10 top-0 right-0"
            hidden=move || delete.with(|d| d.0.is_none())
            on:click=move |_| delete.update(|d| d.0 = None)
        />
    }
}

// TODO: Render this with a <For> element instead of a vector.
fn backpack_view(cx: Scope, pc: &PC) -> View {
    let mut counter = 1;
    pc.inventory
        .iter()
        .enumerate()
        .map(|(item_i, item)| {
            let item = item.clone();
            let weight = item.weight;
            let (i, i_end) = if weight > 1 {
                (counter, Some(counter + weight - 1))
            } else {
                (counter, None)
            };
            counter += weight;
            let base = match item.spec {
                ItemSpec::Buff(_) => view! { cx, <BuffItem id=(item_i as u32) item=&item /> },
                _ => view! { cx, <BackpackItem item=&item /> },
            };
            let stacks = match item.spec.as_stackable() {
                Some((curr, max)) => view! { cx, <StackBtn i=item_i curr=*curr max=*max /> },
                None => ().into_view(cx),
            };
            view! {
                cx,
                <WeightBtn i i_end item_i/>
                { base }
                { stacks }
            }
        })
        .collect_view(cx)
}

#[component]
fn BackpackItem<'a>(cx: Scope, item: &'a Item) -> impl IntoView {
    const SHARED: &str = "border-zinc-700 ";
    const NO_STACKS: &str = "rounded border-y-2 border-r-2 col-span-7";
    const STACKS: &str = "border-y-2 col-span-6";
    let class = match item.spec.as_stackable() {
        Some(_) => concatcp!(SHARED, STACKS),
        None => concatcp!(SHARED, NO_STACKS),
    };
    view! {
        cx,
        <div class=class>
            { item.into_view(cx) }
        </div>
    }
}
