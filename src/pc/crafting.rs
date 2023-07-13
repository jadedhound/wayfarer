use leptos::*;

use crate::pc::PC;
use crate::utils::read_context;

#[component]
pub fn Crafting(cx: Scope) -> impl IntoView {
    let _pc = read_context::<PC>(cx);

    view! {
        cx,
        "Crafting"
    }
}
