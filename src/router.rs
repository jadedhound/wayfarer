use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::errors::*;
use crate::roster::*;
use crate::settings::*;

async fn initial_load(cx: Scope) -> bool {
    match indxdb::new("wf").await {
        Ok(db) => {
            provide_context(cx, Rc::new(db));
            true
        }
        Err(e) => {
            FatalErr::report(cx, "indxdb loading", e);
            false
        }
    }
}

#[component]
pub fn RouterScout(cx: Scope) -> impl IntoView {
    let load_assets = create_resource(cx, || (), move |_| async move { initial_load(cx).await });
    view! { cx,
        {move || match load_assets.read(cx) {
            None => view! { cx, }.into_view(cx),
            Some(success) => if success {
                view! { cx, <MainRouter /> }.into_view(cx)
            } else {
                view! { cx, <FatalPg /> }.into_view(cx)
            }
        }}
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
