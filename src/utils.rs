use std::fmt::Display;
use std::rc::Rc;

use leptos::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use simple_index::Database;

// -----------------------------------
// SIMPLE FUNCTIONS
// -----------------------------------

/// Get a read/write signal that has already been provided
pub fn rw_context<T>(cx: Scope) -> RwSignal<T> {
    use_context::<RwSignal<T>>(cx).unwrap()
}

/// Capitalises the first letter of a given string.
pub fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Flattens and joins a `vec` of string into a single string.
pub fn flat_concat(v: Vec<Option<String>>, join: &'static str) -> Option<String> {
    v.into_iter().flatten().reduce(|mut acc, e| {
        acc.push_str(join);
        acc.push_str(&e);
        acc
    })
}

// -----------------------------------
// INDEXEDDB
// -----------------------------------

/// Commit key and val to db
/// Currently cleanest way to combine result and async.
async fn to_db<T>(db: Rc<Database>, key: &str, val: &T) -> Result<(), simple_index::Error>
where
    T: Serialize,
{
    let mut tx = db.begin(false)?;
    tx.set(key, val).await?;
    tx.commit().await
}

async fn from_db<T>(db: Rc<Database>, key: &str) -> Result<T, simple_index::Error>
where
    T: DeserializeOwned,
{
    let mut tx = db.begin(true)?;
    let val = tx.get(key).await?;
    tx.close().await?;
    Ok(val)
}

/// Creates read and write signals for given value and
/// commits it to indexeddb when the value changes.
pub async fn provide_saved<K, F, T>(cx: Scope, key: K, default: F)
where
    K: Display,
    F: Fn() -> T,
    T: Serialize + DeserializeOwned + Clone + 'static,
{
    let key = key.to_string();
    let db = use_context::<Rc<Database>>(cx).unwrap();
    let val = from_db(db, &key).await.unwrap_or_else(|_| default());
    // Create signals to change values
    let rw = create_rw_signal(cx, val);
    provide_context(cx, rw);

    // Save to indexeddb when value is changed
    create_effect(cx, move |_| {
        let key = key.to_string();
        let val = rw.get();
        spawn_local(async move {
            let db = use_context::<Rc<Database>>(cx).unwrap();
            to_db(db, &key, &val)
                .await
                .unwrap_or_else(|e| log::error!("{e}"));
        });
    });
}

/// Adds a `+` to positive values.
pub fn add_operator(x: i32) -> String {
    if x > -1 {
        format!("+{x}")
    } else {
        x.to_string()
    }
}
