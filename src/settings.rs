use leptos::*;
use leptos_router::*;

#[component]
pub fn Settings(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class= "px-2 py-4 flex border-b-2 border-wfborder">
            <h3 class= "grow"> "SETTINGS" </h3>
            <A href= "/" class= "flex items-center">
                <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
            </A>
        </div>
        <div class= "h-full p-2 flex justify-center">
            "Coming Soon..."
        </div>
    }
}
