use leptos::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Login {
    pub user: String,
    pub pass: String,
}

#[component]
pub fn LoginPg(cx: Scope) -> impl IntoView {
    const IN_BOX: &str = "font-sans rounded-xl bg-zinc-700 p-2 mt-2";
    view! { cx,
        <div class="flex flex-col h-cover items-center justify-center text-center px-4">
            <h1> "Wayfarer" </h1>
            <input class=IN_BOX placeholder="User" />
            <input class=IN_BOX type="password" placeholder="Password" />
            <button class= "font-sans-condensed mt-4 bg-wfblue rounded-xl w-36 py-2"> "Create" </button>
        </div>
    }
}
