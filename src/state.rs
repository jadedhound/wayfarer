use std::rc::Rc;

use leptos::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppState {}

#[derive(Serialize, Deserialize)]
pub struct SessionState {}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCState(Vec<PC>);

#[derive(Serialize, Deserialize, Clone)]
pub struct PC {
    name: String,
}

pub async fn from_db<T>(tx: &mut indxdb::Tx, key: &str) -> Result<T, indxdb::Error>
where
    T: Default + DeserializeOwned,
{
    let opt_val = tx.get(bincode::serialize(key).unwrap()).await?;
    match opt_val {
        Some(val_enc) => {
            if val_enc.is_empty() {
                Ok(T::default())
            } else {
                Ok(bincode::deserialize(&val_enc).unwrap())
            }
        }
        None => Ok(T::default()),
    }
}

pub async fn to_db<T>(db: Rc<indxdb::Db>, key: &str, val: &T) -> Result<(), indxdb::Error>
where
    T: Default + Serialize,
{
    let mut tx = db.begin(true).await?;
    log::info!("began new for {key}, is closed {}", tx.closed());
    tx.set(
        bincode::serialize(key).unwrap(),
        bincode::serialize(val).unwrap(),
    )
    .await?;
    log::info!("set val");
    tx.commit().await
}
