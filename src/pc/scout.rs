use leptos::*;
use leptos_router::*;

use super::journal::Journal;
use super::navbar::pc_navbar;
use super::session::Session;
use super::{update, PC};
use crate::error::{fatal_page, Error};
use crate::indexeddb::DBKey;
use crate::lobby::PCList;
use crate::utils::fetch;
use crate::utils::rw_utils::RwUtils;

pub fn pc_scout() -> impl IntoView {
    let loading = fetch(move || async {
        let id = use_location().pathname.with_untracked(|location| {
            let id_str = location.split('/').nth(2)?;
            id_str.parse::<usize>().ok()
        })?;
        let pc_basic = PCList::expect().with_untracked(|list| list.0.get(id).cloned())?;
        // Get from IndexedDB.
        DBKey::PCJournal(id).provide(Journal::default).await;
        DBKey::PC(id).provide(|| PC::from(pc_basic)).await;
        // Create session.
        Session::provide();
        // Init update hooks.
        update::updater();
        Some(())
    });

    move || {
        loading.get().map(|pc_loaded| match pc_loaded {
            Some(_) => pc_navbar().into_view(),
            None => fatal_page(Error::PCNotFound).into_view(),
        })
    }
}
