use leptos::*;
use leptos_router::*;

use super::session::PCSession;
use super::PC;
use crate::error::{Error, FatalErr, FatalPg};
use crate::pc::navbar::NavBarWithOutlet;
use crate::state::PCList;
use crate::utils::{provide_saved, rw_context};

async fn get_pc(cx: Scope) -> Option<()> {
    let brief = use_params_map(cx).with_untracked(|params| {
        params.get("id").and_then(|id| {
            let id = id.parse::<u64>().ok()?;
            rw_context::<PCList>(cx).with_untracked(|list| list.get(id).cloned())
        })
    })?;
    let new_pc = || {
        let key = brief.name.clone();
        PC::new(cx, key)
    };
    provide_saved(cx, brief.id, new_pc).await;
    PCSession::provide(cx);
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
