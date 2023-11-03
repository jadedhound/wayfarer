use leptos::*;
use leptos_router::*;

use crate::error::*;
use crate::indexeddb;
use crate::indexeddb::DBKey;
use crate::lobby::{lobby, NewPCTimeout, PCList};
use crate::pc::class::view::class;
use crate::pc::combat::combat;
use crate::pc::edit_item::edit_item;
use crate::pc::inventory::inventory;
use crate::pc::journal::edit_note::edit_note;
use crate::pc::journal::overview::journal;
use crate::pc::realm::realm;
use crate::pc::realm::sell::sell;
use crate::pc::realm::shop::delegate::shop_delegate;
use crate::pc::scout::pc_scout;
use crate::rand::Rand;
use crate::settings::settings;
use crate::utils::fetch;
use crate::utils::rw_utils::RwUtils;
use crate::views::delete_confirm::{delete_confirm_modal, DeleteModal};
use crate::views::modal::{modal_grey_screen, ModalState};
use crate::views::revealer::{revealer_screen, Revealer};
use crate::views::toast::{toast_notification, Toast};

pub fn main_router() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path= "/" view=critical_assets>
                    <Route path= "settings" view=settings />
                    <Route path= "" view=secondary_assets>
                        <Route path= "" view=lobby />
                        <Route path= "pc/:id" view=pc_scout >
                            <Route path= "overview" view=combat />
                            <Route path= "inventory" view=inventory />
                            <Route path= "edit_item/:id" view=edit_item />
                            <Route path= "journal" view=journal />
                            <Route path= "edit_note/:id" view=edit_note />
                            <Route path= "realm" view=realm />
                            <Route path= "realm/buy/:repr" view=shop_delegate />
                            <Route path= "realm/sell" view=sell />
                            <Route path= "class" view=class />
                            <Route path= "*any" view=|| view!{ <Redirect path="overview" /> }/>
                        </Route>
                    </Route>
                </Route>
                <Route path= "/*any" view=|| view!{ <Redirect path="" /> } />
            </Routes>
        </Router>
    }
}

fn critical_assets() -> impl IntoView {
    let load_assets = fetch(move || async {
        // IndexedDB
        indexeddb::provide().await?;
        // Popup modals
        ModalState::provide();
        DeleteModal::provide();
        Toast::provide();
        Revealer::provide();
        Ok(())
    });

    move || {
        load_assets
            .get()
            .map(|result| match result {
                Ok(_) => main().into_view(),
                Err(err) => fatal_page(err).into_view(),
            })
            .into_view()
    }
}

fn secondary_assets() -> impl IntoView {
    let load_assets = fetch(move || async {
        Rand::provide();
        DBKey::PCList.provide(PCList::default).await;
        DBKey::NewPCTimeout.provide(NewPCTimeout::default).await;
        Some(())
    });

    move || load_assets.get().flatten().map(|_| Outlet()).into_view()
}

fn main() -> impl IntoView {
    view! {
        <Outlet />
        // Z-10
        { toast_notification }
        // Z-20
        { modal_grey_screen }
        // Z-30
        { revealer_screen }
        // Ever present delete modal
        { delete_confirm_modal }
    }
}
