use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::error::*;
use crate::lobby::{lobby, NewPCTimeout, PCList};
use crate::pc::class_view::class;
use crate::pc::inventory::inventory;
use crate::pc::journal::journal;
use crate::pc::overview::overview;
use crate::pc::realm::realm;
use crate::pc::realm::shop::shop;
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
                        <Route path= "overview" view=overview />
                        <Route path= "inventory" view=inventory />
                        <Route path= "journal" view=journal />
                        <Route path= "realm" view=realm />
                        <Route path= "realm/shop" view=shop />
                        <Route path= "class" view=class />
                        <Route path= "*any" view=|| view!{ <Redirect path="overview" /> }/>
                    </Route>
                </Route>
                <Route path= "/*any" view=|| view!{ <Redirect path="" /> } />
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
    provide_saved("new_pc_timeout", || NewPCTimeout(0.0)).await;

    // Popup modals
    provide_context(create_rw_signal(ModalState::new()));
    provide_context(create_rw_signal(Toast::default()));
    provide_context(create_rw_signal(Revealer(None)));
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
