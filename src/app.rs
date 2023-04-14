use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <AppRouter />
    }
}

#[component]
fn AppRouter(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path="" view=move |cx| view! { cx, <Home/> }/>
                <Route path="/*any" view=|cx| view! { cx, <NotFound/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col h-full justify-center text-center">
            <h1> "Wayfarer" </h1>
        </div>
    }
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="">
            <h1>"Not Found"</h1>
        </div>
    }
}
