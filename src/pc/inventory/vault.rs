use leptos::*;

use crate::pc::PC;
use crate::utils::read_context;

#[component]
pub fn Vault(cx: Scope) -> impl IntoView {
    let _pc = read_context::<PC>(cx);

    view! {
        cx,
        "Vault"
    }
}
