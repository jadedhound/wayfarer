use leptos::*;
use leptos_router::*;

use crate::error::*;
use crate::lobby::{lobby, NewPCTimeout, PCList};
use crate::pc::class::view::class;
use crate::pc::combat::combat;
use crate::pc::inventory::Inventory;
use crate::pc::journal::Journal;
use crate::pc::realm::{realm, Sell, ShopView};
use crate::pc::scout::PCScout;
use crate::rand::Rand;
use crate::utils::db;
use crate::utils::db::DBKey;
use crate::utils::rw_utils::RwUtils;
use crate::views::delete_confirm::{delete_confirm_modal, DeleteModal};
use crate::views::modal::{modal_grey_screen, ModalState};
use crate::views::revealer::{revealer_screen, Revealer};
use crate::views::toast::{toast_notification, Toast};

pub fn main_router() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path= "/" view=load_assets>
                    <Route path= "" view=lobby />
                    <Route path= "/pc/:id" view=PCScout >
                        <Route path= "overview" view=combat />
                        <Route path= "inventory" view=Inventory />
                        <Route path= "journal" view=Journal />
                        <Route path= "realm" view=realm />
                        <Route path= "realm/buy/:repr" view=ShopView />
                        <Route path= "realm/sell" view=Sell />
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
    Rand::provide();
    // IndexedDB
    db::provide().await?;
    DBKey::PCList.provide(PCList::default).await;
    DBKey::NewPCTimeout.provide(NewPCTimeout::default).await;
    // Popup modals
    ModalState::provide();
    DeleteModal::provide();
    Toast::provide();
    Revealer::provide();
    Ok(())
}

fn load_assets() -> impl IntoView {
    let load_assets = create_local_resource(|| (), move |_| async move { init_assets().await });
    move || {
        load_assets
            .get()
            .map(|result| match result {
                Ok(_) => main().into_view(),
                Err(err) => view! { <FatalPage err /> }.into_view(),
            })
            .into_view()
    }
}

fn main() -> impl IntoView {
    view! {
        <main class= "flex flex-col p-2 gap-y-2 min-h-screen">
            <Outlet />
        </main>
        { modal_grey_screen }
        { revealer_screen }
        { toast_notification }
        { delete_confirm_modal }
    }
}
