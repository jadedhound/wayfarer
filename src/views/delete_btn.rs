use leptos::*;

use crate::icons;
use crate::views::revealer::Revealer;

pub fn delete_btn<F>(origin: char, id: usize, f: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! {
        <div hidden=move || !Revealer::state(origin, id)>
            <div class= "absolute w-full -translate-y-4 z-40">
                <div class= "w-5 mx-auto fill-red-900 translate-y-px" inner_html=icons::TRIANGLE />
                <button
                    class= "btn flex-center bg-red-900 w-full rounded gap-1 p-2"
                    on:click=move |_| f()
                >
                    <div class= "w-5 pb-1" inner_html=icons::TRASH />
                    <div class= "font-tight">
                            "DELETE"
                    </div>
                </button>
            </div>
        </div>
    }
}
