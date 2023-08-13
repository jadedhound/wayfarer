use std::fmt::Display;
use std::rc::Rc;

use leptos::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use simple_index::{Database, Error as SIError};

/// Commit key and val to db
/// Currently cleanest way to combine result and async.
async fn set<T>(db: Rc<Database>, key: &str, val: &T) -> Result<(), SIError>
where
    T: Serialize,
{
    let mut tx = db.begin(false)?;
    tx.set(key, val).await?;
    tx.commit().await
}

async fn get<T>(db: Rc<Database>, key: &str) -> Result<T, SIError>
where
    T: DeserializeOwned,
{
    let mut tx = db.begin(true)?;
    let val = tx.get(key).await?;
    tx.close().await?;
    Ok(val)
}

/// Remove an array of keys from the db.
pub async fn remove<T, K>(cx: Scope, key_arr: T) -> Result<(), SIError>
where
    T: Iterator<Item = K>,
    K: Display,
{
    let db = use_context::<Rc<Database>>(cx).unwrap();
    let mut tx = db.begin(false)?;
    tx.remove_many(key_arr).await?;
    tx.commit().await
}

/// Creates read and write signals for given value and
/// commits it to indexeddb when the value changes.
pub async fn provide_saved<K, F, T>(cx: Scope, key: K, default: F)
where
    K: Display,
    F: FnOnce() -> T,
    T: Serialize + DeserializeOwned + Clone + 'static,
{
    let key = key.to_string();
    let db = use_context::<Rc<Database>>(cx).unwrap();
    let val = get(db, &key).await.unwrap_or_else(|_| default());
    // Create signals to change values
    let rw = create_rw_signal(cx, val);
    provide_context(cx, rw);

    // Save to indexeddb when value is changed
    create_effect(cx, move |_| {
        let key = key.to_string();
        let val = rw.get();
        spawn_local(async move {
            let db = use_context::<Rc<Database>>(cx).unwrap();
            set(db, &key, &val)
                .await
                .unwrap_or_else(|e| log::error!("{e}"));
        });
    });
}
