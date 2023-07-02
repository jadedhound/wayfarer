use leptos::*;
use leptos_router::*;

use crate::errors::*;
use crate::roster::*;
use crate::settings::*;

#[derive(Clone)]
pub struct IsLoaded(bool);

#[component]
pub fn RouterScout(cx: Scope) -> impl IntoView {
    let (get_loaded, set_loaded) = create_signal(cx, IsLoaded(false));
    provide_context(cx, set_loaded);
    view! {
        cx,
        {if get_loaded.get().0 {
            view!{ cx, <MainRouter /> }.into_view(cx)
        } else {
            view!{ cx, <LoadingPg /> }.into_view(cx)
        }
        }
    }
}

#[component]
fn LoadingPg(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <h1 class= "flex items-center justify-center h-cover"> "Loading" </h1>
    }
}

#[component]
pub fn MainRouter(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path= "" view=move |cx| view! { cx, <Roster /> }/>
                <Route path= "/settings" view=move |cx| view! { cx, <Settings /> }/>
                <Route path= "/*any" view=|cx| view! { cx, <NotFound/> }/>
            </Routes>
        </Router>
    }
}
