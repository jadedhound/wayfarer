use leptos::*;

#[component]
pub fn Journal(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class= "flex flex-col px-2">
            <h3 class= "text-center border-b border-teal-700"> "Notes" </h3>
            <div class= "h-12 rounded w-full bg-zinc-800 flex-centered mt-2">
                "New"
            </div>
        </div>
    }
}
