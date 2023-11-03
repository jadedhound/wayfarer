use std::fmt::Display;

use idb::Query;
use js_sys::{ArrayBuffer, Uint8Array};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;

use super::convert::{rust_from_js, rust_to_js};
use super::DBError;

pub struct Transaction {
    tx: Option<idb::Transaction>,
    store: idb::ObjectStore,
}

impl Transaction {
    /// Create a new transaction.
    pub(crate) fn new(tx: idb::Transaction, store: idb::ObjectStore) -> Self {
        Self {
            tx: Some(tx),
            store,
        }
    }

    /// Get the value for a given key.
    pub async fn get<K, V>(&mut self, key: K) -> Result<V, DBError>
    where
        K: Display,
        V: DeserializeOwned,
    {
        let key = key.to_string();
        let val = self
            .store
            .get(rust_to_js(&key)?)
            .await?
            .ok_or(DBError::NoValue(key))?;
        rust_from_js(val)
    }

    /// Remove a given key and its value.
    pub async fn remove<K: Display>(&mut self, key: K) -> Result<(), DBError> {
        let key = key.to_string();
        self.store.delete(Query::Key(rust_to_js(key)?)).await?;
        Ok(())
    }

    /// Save a key and value pair int the DB.
    pub async fn set<K, V>(&mut self, key: K, val: V) -> Result<(), DBError>
    where
        K: Display,
        V: Serialize,
    {
        self.store
            .put(&rust_to_js(val)?, Some(&rust_to_js(key.to_string())?))
            .await?;
        Ok(())
    }

    /// Delete all values from the database.
    pub async fn delete_all(&mut self) -> Result<(), DBError> {
        self.store.clear().await?;
        Ok(())
    }

    /// Gets the size of the database.
    pub async fn size(&mut self) -> Result<u64, DBError> {
        let mut size = 0;
        let keys = self.store.get_all_keys(None, None).await?;
        for key in keys {
            let value = self.store.get(key).await?.ok_or(DBError::NoValueNoKey)?;
            let value = js_sys::Uint8Array::from(value).to_vec();
            size += value.len() as u64;
        }
        Ok(size)
    }

    /// Exports all values as a byte array.
    pub async fn export(&mut self) -> Result<Vec<u8>, DBError> {
        // The blob just takes the raw bincode saved indexeddb.
        // So keys and values are left as bincode without serialising them into
        // proper values.
        let mut blob = Vec::new();
        let keys = self.store.get_all_keys(None, None).await?;
        for key in keys {
            let value = self
                .store
                .get(key.clone())
                .await?
                .ok_or(DBError::NoValueNoKey)?;
            let value = js_sys::Uint8Array::from(value).to_vec();
            let key = Uint8Array::new(&ArrayBuffer::from(key)).to_vec();
            blob.push((key, value));
        }
        // The hashmap is serialised once more to ensure a single byte array is returned.
        let blob = bincode::serialize(&blob)?;
        Ok(blob)
    }

    /// Imports previously exported values.
    pub async fn import(&mut self, export: Vec<u8>) -> Result<(), DBError> {
        let convert = |value: Vec<u8>| JsValue::from(js_sys::Uint8Array::from(value.as_slice()));
        let blob: Vec<(Vec<u8>, Vec<u8>)> = bincode::deserialize(&export)?;
        for (key, value) in blob {
            let (key, value) = (convert(key), convert(value));
            self.store.put(&value, Some(&key)).await?;
        }
        Ok(())
    }

    /// Close the transaction without changing anything.
    pub async fn close(mut self) -> Result<(), DBError> {
        Ok(self.tx.take().unwrap().done().await?)
    }

    /// Close the transaction and writing to results to the DB.
    pub async fn commit(mut self) -> Result<(), DBError> {
        Ok(self.tx.take().unwrap().commit().await?)
    }
}
