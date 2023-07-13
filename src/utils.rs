use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use leptos::*;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use simple_index::Database;

mod fuzzy_match;

pub use fuzzy_match::*;
use strum::EnumCount;

pub type LazyHash<T> = Lazy<HashMap<String, T>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct EnumMap<V>(pub Vec<V>);
impl<V> EnumMap<V>
where
    V: Clone,
{
    pub fn new<K>(v: V) -> Self
    where
        K: EnumCount,
    {
        EnumMap(vec![v; K::COUNT])
    }

    pub fn get<K>(&self, key: K) -> &V
    where
        K: EnumIndex,
    {
        &self.0[key.index()]
    }

    pub fn get_mut<K>(&mut self, key: K) -> &mut V
    where
        K: EnumIndex,
    {
        &mut self.0[key.index()]
    }
}

pub trait EnumIndex {
    fn index(&self) -> usize;
}

// -----------------------------------
// SIMPLE FUNCTIONS
// -----------------------------------

/// Joins an array of optional strings.
pub fn concat_some_str(arr: Vec<Option<String>>, join: &'static str) -> String {
    arr.into_iter()
        .flatten()
        .reduce(|mut acc, e| {
            acc.push_str(join);
            acc.push_str(&e);
            acc
        })
        .unwrap_or_default()
}

/// Captilises the first letter in s.
pub fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Get a read signal that has already been provided
pub fn read_context<T>(cx: Scope) -> ReadSignal<T> {
    use_context::<ReadSignal<T>>(cx).unwrap()
}

/// Get a write signal that has already been provided
pub fn write_context<T>(cx: Scope) -> WriteSignal<T> {
    use_context::<WriteSignal<T>>(cx).unwrap()
}

/// Get a read/write signal that has already been provided
pub fn rw_context<T>(cx: Scope) -> RwSignal<T> {
    use_context::<RwSignal<T>>(cx).unwrap()
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
pub async fn provide_saved<K, T>(cx: Scope, key: K, default: T)
where
    K: Display,
    T: Serialize + DeserializeOwned + Clone + 'static,
{
    let key = key.to_string();
    let db = use_context::<Rc<Database>>(cx).unwrap();
    let val = from_db(db, &key).await.unwrap_or(default);
    // Create signals to change values
    let (rs, ws) = create_signal(cx, val);
    provide_context(cx, rs);
    provide_context(cx, ws);

    // Save to indexeddb when value is changed
    create_effect(cx, move |_| {
        let key = key.clone();
        let val = rs.get();
        spawn_local(async move {
            let db = use_context::<Rc<Database>>(cx).unwrap();
            to_db(db, &key, &val)
                .await
                .unwrap_or_else(crate::error::log);
        });
    });
}

// -----------------------------------
// STRING HELPERS
// -----------------------------------

pub trait StrPlus {
    fn plus(&self, s: &str) -> String;
}

impl StrPlus for str {
    fn plus(&self, s: &str) -> String {
        format!("{self} {s}")
    }
}

// -----------------------------------
// VIEWS
// -----------------------------------

#[component]
pub fn Modal<F>(cx: Scope, children: Children, hidden: F) -> impl IntoView
where
    F: Fn() -> bool + 'static,
{
    view! {
        cx,
        <div class="relative z-10" hidden=move || hidden()>
            <div class="fixed inset-0 bg-zinc-700/75 transition-opacity"></div>
            <div class="fixed inset-0 z-10 overflow-y-auto">
                {children(cx)}
            </div>
        </div>
    }
}
