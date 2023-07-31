use const_format::concatcp;
use leptos::*;

use crate::items::item_specs::ItemSpec;
use crate::pc::PC;
use crate::utils::rw_context;
use crate::{css, svg};

struct ChgStack(bool);

#[component]
pub(super) fn StackBtn(cx: Scope, i: usize, curr: u8, max: u8) -> impl IntoView {
    const CHG_BTN: &str = " absolute flex-centered w-10 h-10 z-20";
    let chg_stack = create_rw_signal(cx, ChgStack(false));
    provide_context(cx, chg_stack);
    let hidden = move || chg_stack.with(|x| !x.0);

    view! {
        cx,
        <div class= "relative">
            <div hidden=move|| { hidden() || curr == max }>
                <button
                    class=concatcp!(css::BTN, CHG_BTN, " top-0 -translate-y-12")
                    on:click=move |_| change_stack(cx, i, 1)
                >
                    <div class= "svg w-6" inner_html=svg::PLUS />
                </button>
            </div>
            <button
                class= "flex flex-col justify-center text-center bg-sky-900 rounded-r px-1 h-full w-full"
                on:click=move |_| chg_stack.update(|x| x.0 = true)
            >
                <span class= "border-b w-full"> { curr } </span>
                <span class= "w-full"> { max } </span>
            </button>
            <div hidden=move|| { hidden() || curr < 2}>
                <button
                    class=concatcp!(css::BTN, CHG_BTN, " bottom-0 translate-y-12")
                    on:click=move |_| change_stack(cx, i, -1)
                >
                    <div class= "svg w-6" inner_html=svg::MINUS />
                </button>
            </div>
        </div>
        <div
            class= "cursor-pointer fixed z-10 inset-0"
            on:click=move |_| chg_stack.update(|x| x.0 = false)
            hidden=hidden
        />
    }
}

fn change_stack(cx: Scope, i: usize, by: i16) {
    let pc = rw_context::<PC>(cx);
    pc.update(|pc| {
        let item = &mut pc.inventory[i];
        let (curr, max) = item.spec.as_stackable().unwrap();
        let new_curr = u8::try_from(*curr as i16 + by).unwrap();
        item.spec = ItemSpec::Stackable(new_curr, *max);
    })
}
