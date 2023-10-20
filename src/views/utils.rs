use leptos::*;

/// A `div` that grows within a flexbox.
#[component]
pub fn Spacer() -> impl IntoView {
    view! {
        <div class= "psuedo h-px w-px grow" />
    }
}
