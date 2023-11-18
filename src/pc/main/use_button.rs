use leptos::*;

use crate::icons;
use crate::utils::counter::Counter;

pub fn use_button<C, F>(count: C, onclick: F, use_on_zero: bool) -> impl IntoView
where
    C: Fn() -> Counter + Copy + 'static,
    F: Fn() + Copy + 'static,
{
    let button = move || {
        view! {
            <button
                class= "btn bg-sky-800 w-full disabled:!text-sky-800 disabled:!border-sky-900"
                disabled=move || count().curr < 1 && !use_on_zero
                on:click=move |_| onclick()
            >
                "USE"
            </button>
        }
    };
    let range = move || {
        let max = count().max;
        let curr = move || count().curr;
        let steps = (1..max)
            .map(|_| view! { <div class= "w-1 psuedo bg-sky-950" /> })
            .collect_view();

        view! {
            <div class= "flex gap-1">
                <button
                    class= "btn bg-sky-800 disabled:!fill-sky-800 disabled:!stroke-sky-800 disabled:!border-sky-900"
                    on:click=move |_| onclick()
                    disabled=move || curr() < 1
                >
                    <div class= "w-6" inner_html=icons::BOLT />
                </button>
                <div class= "relative w-12 grow">
                    <input
                        class= "range [color:theme(colors.sky.800)] bg-sky-950 w-full h-full pointer-events-none"
                        type= "range"
                        min=0
                        max=max
                        value=curr
                        step=1
                    />
                    <div class= "absolute w-full flex justify-evenly top-0 h-full">
                        { steps }
                    </div>
                </div>
            </div>
        }
    };

    if count().max > 1 {
        range().into_view()
    } else {
        button().into_view()
    }
}
