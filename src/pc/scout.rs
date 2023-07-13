use leptos::*;
use leptos_router::*;

use super::{PCSession, PC};
use crate::error::{Error, FatalErr, FatalPg};
use crate::pc::navbar::NavBarWithOutlet;
use crate::state::PCList;
use crate::utils::{provide_saved, read_context};

async fn get_pc(cx: Scope) -> Option<()> {
    let brief = use_params_map(cx).with_untracked(|params| {
        params.get("id").and_then(|id| {
            let id = id.parse::<u64>().ok()?;
            read_context::<PCList>(cx).with_untracked(|list| list.get(id).cloned())
        })
    })?;
    provide_saved(cx, brief.id, PC::new(cx, brief.name)).await;
    let (session, session_set) = create_signal(cx, PCSession::new(cx));
    provide_context(cx, session);
    provide_context(cx, session_set);
    Some(())
}

#[component]
pub fn PCScout(cx: Scope) -> impl IntoView {
    let loading = create_resource(cx, || (), move |_| async move { get_pc(cx).await });

    view! {
        cx,
        {move || match loading.read(cx) {
            None => view!{ cx, }.into_view(cx),
            Some(get_pc) => match get_pc {
                Some(_) => view!{ cx, <NavBarWithOutlet /> }.into_view(cx),
                None => {
                    FatalErr::provide(cx, "PCScout", Error::PCNotFound);
                    view!{ cx, <FatalPg /> }.into_view(cx)
                },
            }
        }}
    }
}
