use std::rc::Rc;

use leptos::logging::error;
use leptos::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use simple_index::{Database, Error as SIError};

use crate::error::Error;

#[derive(Clone, Copy)]
pub enum DBKey {
    PCList,
    NewPCTimeout,
    PC(usize),
    PCJournal(usize),
}

impl std::fmt::Display for DBKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBKey::PCList => write!(f, "pc_list"),
            DBKey::NewPCTimeout => write!(f, "new_pc_timeout"),
            DBKey::PC(id) => write!(f, "pc_{id}"),
            DBKey::PCJournal(id) => write!(f, "pc_journal_{id}"),
        }
    }
}

impl DBKey {
    /// Creates a read write signal for a given key and
    /// commits it to indexeddb when the value changes.
    pub async fn provide<F, T>(&self, default: F)
    where
        F: FnOnce() -> T,
        T: Serialize + DeserializeOwned + Clone + 'static,
    {
        let key = self.to_string();
        let db = expect_context::<Rc<Database>>();
        let value = get(&db, &key).await.unwrap_or_else(|_| default());
        // Create signals to change values
        let rw = RwSignal::new(value);
        provide_context(rw);

        // Save to indexeddb when value is changed
        create_effect(move |_| {
            let value = rw.get();
            let (db, key) = (db.clone(), key.clone());
            spawn_local(async move {
                set(&db, &key, &value)
                    .await
                    .unwrap_or_else(|e| error!("ProvideSaved: {e}"));
            });
        });
    }

    /// Remove an array of keys from the db.
    pub async fn remove(&self) -> Result<(), SIError> {
        let db = expect_context::<Rc<Database>>();
        let mut tx = db.begin(false)?;
        tx.remove(self.to_string()).await?;
        tx.commit().await
    }
}

pub async fn provide() -> Result<(), Error> {
    let db = simple_index::new().await?;
    provide_context(Rc::new(db));
    Ok(())
}

/// Commit key and val to db
/// Currently cleanest way to combine result and async.
async fn set<T>(db: &Rc<Database>, key: &str, val: &T) -> Result<(), SIError>
where
    T: Serialize,
{
    let mut tx = db.begin(false)?;
    tx.set(key, val).await?;
    tx.commit().await
}

async fn get<T>(db: &Rc<Database>, key: &str) -> Result<T, SIError>
where
    T: DeserializeOwned,
{
    let mut tx = db.begin(true)?;
    let val = tx.get(key).await?;
    tx.close().await?;
    Ok(val)
}
