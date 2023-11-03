use idb::{Factory, ObjectStoreParams, TransactionMode};

use super::dberror::DBError;
use super::transaction::Transaction;

const STORE: &str = "kv";

pub async fn new() -> Result<Database, DBError> {
    let factory = Factory::new()?;
    let mut open_request = factory.open(STORE, Some(1))?;
    // Create store if it doesn't exist
    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();
        let store_params = ObjectStoreParams::new();
        database.create_object_store(STORE, store_params).unwrap();
    });

    Ok(Database {
        inner: open_request.await?,
    })
}

pub struct Database {
    inner: idb::Database,
}

impl Database {
    /// Begins a transaction.
    pub fn begin(&self, read_only: bool) -> Result<Transaction, DBError> {
        let mode = match read_only {
            true => TransactionMode::ReadOnly,
            false => TransactionMode::ReadWrite,
        };
        let tx = self.inner.transaction(&[STORE], mode)?;
        let store = tx.object_store(STORE)?;
        Ok(Transaction::new(tx, store))
    }
}
