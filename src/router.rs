use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::error::*;
use crate::lobby::*;
use crate::pc::*;
use crate::rand::init_rand;
use crate::settings::*;
use crate::state::PCList;
use crate::utils::provide_saved;

#[component]
pub fn MainRouter(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path= "" view=move |cx| view! { cx, <Lobby /> }/>
                <Route path= "/settings" view=move |cx| view! { cx, <Settings /> }/>
                <Route path= "/pc/:id" view=move |cx| view! { cx, <PCScout /> }>
                    <Route path="" view=|cx| view! {cx, <Overview /> }/>
                    <Route path= "/basics" view=|cx| view! {cx, <Overview /> }/>
                    <Route path= "/crafting" view=|cx| view! { cx, <Crafting /> }/>
                    <Route path= "/inventory" view=|cx| view! { cx, <InvNavbar /> }>
                        <Route path= "" view=|cx| view! { cx, <Inventory /> }/>
                        <Route path= "/vault" view=|cx| view! { cx, <Vault /> }/>
                    </Route>
                    <Route path= "/journal" view=|cx| view! {cx, <Journal /> }/>
                    <Route path= "/followers" view=|cx| view! {cx, <Followers /> }/>
                </Route>
                <Route path= "/*any" view=|cx| view! { cx, <NotFound /> }/>
            </Routes>
        </Router>
    }
}

async fn init_assets(cx: Scope) -> Result<(), Error> {
    init_rand(cx);
    let db = simple_index::new().await?;
    provide_context(cx, Rc::new(db));
    provide_saved(cx, "pc_list", PCList::default()).await;
    Ok(())
}

#[component]
pub fn RouterScout(cx: Scope) -> impl IntoView {
    let load_assets = create_resource(
        cx,
        || (),
        move |_| async move {
            init_assets(cx)
                .await
                .map_err(|e| FatalErr::provide(cx, "init_assets", e))
                .is_ok()
        },
    );

    view! { cx,
        {move || match load_assets.read(cx) {
            None => view! { cx, }.into_view(cx),
            Some(loaded) => if loaded {
                view! { cx, <MainRouter /> }.into_view(cx)
            } else {
                view! { cx, <FatalPg /> }.into_view(cx)
            }
        }}
    }
}
