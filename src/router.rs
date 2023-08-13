use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::error::*;
use crate::lobby::{lobby, PCList};
use crate::pc::craft::Craft;
use crate::pc::followers::Followers;
use crate::pc::inventory::Inventory;
use crate::pc::journal::journal;
use crate::pc::overview::Overview;
use crate::pc::scout::pc_scout;
use crate::rand::init_rand;
use crate::settings::*;
use crate::utils::db::provide_saved;
use crate::views::modal::ModalState;
use crate::views::revealer::Revealer;
use crate::views::toast::Toast;

fn main_router(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path= "" view=lobby />
                <Route path= "/settings" view=move |cx| view! { cx, <Settings /> }/>
                <Route path= "/pc/:id" view=pc_scout >
                    <Route path="" view=|cx| view! {cx, <Overview /> }/>
                    <Route path= "/craft" view=|cx| view! { cx, <Craft /> } />
                    <Route path= "/inventory" view=|cx| view! { cx, <Inventory /> } />
                    <Route path= "/journal" view=journal />
                    <Route path= "/followers" view=|cx| view! {cx, <Followers /> }/>
                </Route>
                <Route path= "/*any" view=|cx| fatal_pg(cx, Error::NotFound) />
            </Routes>
        </Router>
    }
}

async fn init_assets(cx: Scope) -> Result<(), Error> {
    // Random generator
    init_rand(cx);
    // IndexedDB
    let db = simple_index::new().await?;
    provide_context(cx, Rc::new(db));
    provide_saved(cx, "pc_list", PCList::default).await;

    // Popup modals
    let modal = create_rw_signal(cx, ModalState::new());
    provide_context(cx, modal);
    let toast = create_rw_signal(cx, Toast::new());
    provide_context(cx, toast);
    let revealer = create_rw_signal(cx, Revealer(None));
    provide_context(cx, revealer);
    Ok(())
}

pub fn router_scout(cx: Scope) -> impl IntoView {
    let load_assets = create_resource(cx, || (), move |_| async move { init_assets(cx).await });

    move || {
        load_assets
            .read(cx)
            .map(|load_result| match load_result {
                Ok(_) => main_router(cx).into_view(cx),
                Err(e) => fatal_pg(cx, e).into_view(cx),
            })
            .into_view(cx)
    }
}
