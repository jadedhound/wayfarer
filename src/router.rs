use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::error::*;
use crate::lobby::{lobby, PCList};
use crate::pc::craft::craft;
use crate::pc::followers::followers;
use crate::pc::inventory::inventory;
use crate::pc::journal::journal;
use crate::pc::overview::overview;
use crate::pc::scout::pc_scout;
use crate::rand::provide_rand;
use crate::utils::db::provide_saved;
use crate::views::modal::ModalState;
use crate::views::revealer::Revealer;
use crate::views::toast::Toast;

pub fn main_router() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path= "/" view=load_assets_view>
                    <Route path= "" view=lobby />
                    <Route path= "/pc/:id" view=pc_scout >
                        <Route path="" view=overview />
                        <Route path= "/craft" view=craft />
                        <Route path= "/inventory" view=inventory />
                        <Route path= "/journal" view=journal />
                        <Route path= "/followers" view=followers />
                    </Route>
                </Route>
                <Route path= "/*any" view=|| fatal_pg(Error::NotFound) />
            </Routes>
        </Router>
    }
}

async fn init_assets() -> Result<(), Error> {
    // Random generator
    provide_rand();
    // IndexedDB
    let db = simple_index::new().await?;
    provide_context(Rc::new(db));
    provide_saved("pc_list", PCList::default).await;

    // Popup modals
    let modal = create_rw_signal(ModalState::new());
    provide_context(modal);
    let toast = create_rw_signal(Toast::new());
    provide_context(toast);
    let revealer = create_rw_signal(Revealer(None));
    provide_context(revealer);
    Ok(())
}

fn load_assets_view() -> impl IntoView {
    let load_assets = create_resource(|| (), move |_| async move { init_assets().await });
    move || {
        load_assets
            .get()
            .map(|load_result| match load_result {
                Ok(_) => Outlet().into_view(),
                Err(e) => fatal_pg(e).into_view(),
            })
            .into_view()
    }
}
