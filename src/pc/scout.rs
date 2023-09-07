use leptos::*;
use leptos_router::*;

use super::journal::PCJournals;
use super::navbar::pc_navbar;
use super::session::PCSession;
use super::{update, PC};
use crate::error::{fatal_pg, Error};
use crate::lobby::PCList;
use crate::utils::db::provide_saved;
use crate::utils::expect_rw;

async fn get_pc(id: Option<usize>) -> Option<()> {
    let id = id?;
    let pc_basic = expect_rw::<PCList>().with_untracked(|list| list.0.get(id).cloned())?;
    // Get from IndexedDB.
    provide_saved(format!("{id}_journals"), PCJournals::default).await;
    provide_saved(id, || PC::from(pc_basic)).await;
    // Create session.
    provide_context(create_rw_signal(PCSession::new()));
    // Init update hooks.
    update::updater();
    Some(())
}

pub fn pc_scout() -> impl IntoView {
    let loading = create_local_resource(
        || use_params_map().with_untracked(|p| p.get("id")?.parse::<usize>().ok()),
        move |id| async move { get_pc(id).await.is_some() },
    );

    move || {
        loading.get().map(|pc_loaded| {
            if pc_loaded {
                pc_navbar().into_view()
            } else {
                fatal_pg(Error::PCNotFound).into_view()
            }
        })
    }
}
