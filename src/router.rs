use std::rc::Rc;

use leptos::*;
use leptos_router::*;
use serde::Serialize;
use simple_index::Database;

use crate::error::*;
use crate::roster::*;
use crate::settings::*;
use crate::state::AppState;
use crate::state::PCState;

#[component]
pub fn MainRouter(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path= "" view=move |cx| view! { cx, <Roster /> }/>
                <Route path= "/settings" view=move |cx| view! { cx, <Settings /> }/>
                <Route path= "/*any" view=|cx| view! { cx, <NotFound /> }/>
            </Routes>
        </Router>
    }
}

async fn to_db<T: Serialize>(
    db: Rc<Database>,
    key: &str,
    val: &T,
) -> Result<(), simple_index::Error> {
    let mut tx = db.begin(false)?;
    tx.set(key, val).await?;
    tx.commit().await
}

fn save_on_change<T>(cx: Scope, key: &'static str, val: T)
where
    T: Serialize + Clone + 'static,
{
    let (rs, ws) = create_signal(cx, val);
    provide_context(cx, rs);
    provide_context(cx, ws);
    create_effect(cx, move |_| {
        let val = rs.get();
        let db = use_context::<Rc<Database>>(cx).unwrap();
        spawn_local(async move {
            to_db(db, key, &val)
                .await
                .unwrap_or_else(|e| log::error!("unable to save: {key}\n{e}"));
        });
    });
}

async fn init_assets(cx: Scope) -> Result<(), Error> {
    let db = simple_index::new().await?;

    // Read state from db
    let mut tx = db.begin(true)?;
    let pc_state: PCState = tx.get("pc_state").await.unwrap_or_default();
    let app_state: AppState = tx.get("app_state").await.unwrap_or_default();
    tx.close().await?;

    // When state changes save to db
    provide_context(cx, Rc::new(db));
    save_on_change(cx, "pc_state", pc_state);
    save_on_change(cx, "app_state", app_state);

    Ok(())
}

#[component]
pub fn RouterScout(cx: Scope) -> impl IntoView {
    let load_assets = create_resource(
        cx,
        || (),
        move |_| async move {
            init_assets(cx)
                .await
                .map_err(|e| FatalErr::provide(cx, "init_assets", e))
                .is_ok()
        },
    );

    view! { cx,
        {move || match load_assets.read(cx) {
            None => view! { cx, }.into_view(cx),
            Some(loaded) => if loaded {
                view! { cx, <MainRouter /> }.into_view(cx)
            } else {
                view! { cx, <FatalPg /> }.into_view(cx)
            }
        }}
    }
}
