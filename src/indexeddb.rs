use std::rc::Rc;

use leptos::logging::error;
use leptos::*;
use leptos_use::signal_debounced;
use serde::de::DeserializeOwned;
use serde::Serialize;

use self::database::Database;
use self::dberror::DBError;
use crate::error::Error;

mod convert;
mod database;
pub mod dberror;
mod transaction;

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
        async fn set<T>(db: &Rc<Database>, key: &str, val: &T) -> Result<(), DBError>
        where
            T: Serialize,
        {
            let mut tx = db.begin(false)?;
            tx.set(key, val).await?;
            tx.commit().await
        }

        async fn get<T>(db: &Rc<Database>, key: &str) -> Result<T, DBError>
        where
            T: DeserializeOwned,
        {
            let mut tx = db.begin(true)?;
            let val = tx.get(key).await?;
            tx.close().await?;
            Ok(val)
        }

        let key = self.to_string();
        let db = expect_context::<Rc<Database>>();
        let value = get(&db, &key).await.unwrap_or_else(|_| default());
        // Create signals to change values
        let rw = RwSignal::new(value);
        provide_context(rw);
        let debouced = signal_debounced(rw, 5.0 * 1000.0);

        // Save to indexeddb when value is changed
        create_effect(move |_| {
            let value = debouced.get();
            let (db, key) = (db.clone(), key.clone());
            spawn_local(async move {
                if let Err(err) = set(&db, &key, &value).await {
                    error!("Unable to save to DB: {err}")
                }
            });
        });
    }
}

pub async fn provide() -> Result<(), Error> {
    let db = database::new().await?;
    provide_context(Rc::new(db));
    Ok(())
}

/// Remove an array of keys from the db.
pub async fn remove(keys: &[DBKey]) -> Result<(), DBError> {
    let db = expect_context::<Rc<Database>>();
    let mut tx = db.begin(false)?;
    for key in keys {
        tx.remove(key.to_string()).await?;
    }
    tx.commit().await
}

/// Get the total size of the current DB.
pub async fn size() -> Result<u64, DBError> {
    let db = expect_context::<Rc<Database>>();
    let mut tx = db.begin(true)?;
    let size = tx.size().await?;
    tx.close().await?;
    Ok(size)
}

/// Export the entire database.
pub async fn export() -> Result<Vec<u8>, DBError> {
    let db = expect_context::<Rc<Database>>();
    let mut tx = db.begin(true)?;
    let data = tx.export().await?;
    tx.close().await?;
    Ok(data)
}

/// Import a foreign database.
pub async fn import(data: Vec<u8>) -> Result<(), DBError> {
    let db = expect_context::<Rc<Database>>();
    let mut tx = db.begin(false)?;
    tx.import(data).await?;
    tx.close().await
}

/// Import a foreign database.
pub async fn delete_all() -> Result<(), DBError> {
    let db = expect_context::<Rc<Database>>();
    let mut tx = db.begin(false)?;
    tx.delete_all().await?;
    tx.close().await
}
