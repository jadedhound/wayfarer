use serde::{de::DeserializeOwned, Deserialize, Serialize};
use simple_index::Database;
use std::rc::Rc;

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

pub async fn to_db<T>(db: Rc<Database>, key: &str, val: &T) -> Result<(), simple_index::Error>
where
    T: Default + Serialize,
{
    let mut tx = db.begin(false)?;
    tx.set(key, val).await?;
    tx.commit().await
}
