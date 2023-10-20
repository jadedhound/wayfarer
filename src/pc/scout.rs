use leptos::*;
use leptos_router::*;

use super::journal::PCJournals;
use super::navbar::PCNavbar;
use super::session::Session;
use super::{update, PC};
use crate::error::{Error, FatalPage};
use crate::lobby::PCList;
use crate::utils::db::DBKey;
use crate::utils::expect_rw;
use crate::utils::rw_utils::RwUtils;

#[component]
pub fn PCScout() -> impl IntoView {
    let params_to_id = || use_params_map().with(|params| params.get("id")?.parse::<usize>().ok());
    let loading = create_local_resource(params_to_id, get_pc);

    move || {
        loading.get().map(|pc_loaded| {
            if pc_loaded.is_some() {
                PCNavbar().into_view()
            } else {
                view! { <FatalPage err=Error::PCNotFound /> }.into_view()
            }
        })
    }
}

async fn get_pc(id: Option<usize>) -> Option<()> {
    let id = id?;
    let pc_basic = expect_rw::<PCList>().with_untracked(|list| list.0.get(id).cloned())?;
    // Get from IndexedDB.
    DBKey::PCJournal(id).provide(PCJournals::default).await;
    DBKey::PC(id).provide(|| PC::from(pc_basic)).await;
    // Create session.
    Session::provide();
    // Init update hooks.
    update::updater();
    Some(())
}
