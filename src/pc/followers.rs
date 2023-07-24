use leptos::*;

#[component]
pub fn Followers(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class= "flex flex-col px-2">
            <h3 class= "text-center border-b border-pink-700"> "Active" </h3>
            <div class= "h-12 rounded w-full bg-zinc-800 flex-centered mt-2">
                "Find Follower"
            </div>
            <h3 class= "text-center border-b border-pink-700 mt-8"> "Roster" </h3>
        </div>
    }
}
