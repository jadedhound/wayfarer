use std::rc::Rc;

use console_log::log;
use leptos::*;
use leptos_router::*;
use serde::Serialize;
use simple_index::Database;

use crate::errors::*;
use crate::roster::*;
use crate::settings::*;
use crate::state::to_db;
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
                <Route path= "/*any" view=|cx| view! { cx, <NotFound/> }/>
            </Routes>
        </Router>
    }
}

fn save_on_change<T>(cx: Scope, key: &'static str, val: T)
where
    T: Default + Serialize + Clone + 'static,
{
    let (rs, ws) = create_signal(cx, val);
    provide_context(cx, rs);
    provide_context(cx, ws);
    create_effect(cx, move |_| {
        use_context::<ReadSignal<T>>(cx).unwrap().track();
        spawn_local(async move {
            let val = use_context::<ReadSignal<T>>(cx).unwrap()();
            let db = use_context::<Rc<Database>>(cx).unwrap();
            match to_db(db, key, &val).await {
                Ok(_) => (),
                Err(e) => log::error!("unable to save: {key}\n{e}"),
            }
        });
    });
}

async fn get_state_from_db(cx: Scope, db: Database) -> Result<(), simple_index::Error> {
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
            match simple_index::new().await {
                Ok(db) => match get_state_from_db(cx, db).await {
                    Ok(_) => true,
                    Err(e) => {
                        FatalErr::report(cx, "get state from indxdb", e);
                        false
                    }
                },
                Err(e) => {
                    FatalErr::report(cx, "indxdb loading", e);
                    false
                }
            }
        },
    );
    view! { cx,
        {move || match load_assets.read(cx) {
            None => view! { cx, }.into_view(cx),
            Some(success) => if success {
                view! { cx, <MainRouter /> }.into_view(cx)
            } else {
                view! { cx, <FatalPg /> }.into_view(cx)
            }
        }}
    }
}
