use leptos::*;
use leptos_router::*;

use super::journal::PCJournals;
use super::navbar::pc_navbar;
use super::session::PCSession;
use super::PC;
use crate::error::{fatal_pg, Error};
use crate::lobby::PCList;
use crate::utils::db::provide_saved;
use crate::utils::rw_context;

async fn get_pc(cx: Scope) -> Option<()> {
    let id = use_params_map(cx).with_untracked(|p| p.get("id")?.parse::<usize>().ok())?;
    let name = rw_context::<PCList>(cx).with_untracked(|list| list.0.get(&id).cloned())?;
    provide_saved(cx, format!("{id}_journals"), PCJournals::default).await;
    provide_saved(cx, id, || PC::new(cx, name)).await;
    PCSession::provide(cx);
    Some(())
}

pub fn pc_scout(cx: Scope) -> impl IntoView {
    let loading = create_resource(
        cx,
        || (),
        move |_| async move { get_pc(cx).await.is_some() },
    );

    move || {
        loading.read(cx).map(|pc_loaded| {
            if pc_loaded {
                pc_navbar(cx).into_view(cx)
            } else {
                fatal_pg(cx, Error::PCNotFound).into_view(cx)
            }
        })
    }
}
