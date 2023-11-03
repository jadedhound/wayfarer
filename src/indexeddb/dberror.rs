#[derive(thiserror::Error, Debug)]
pub enum DBError {
    #[error(transparent)]
    Idb(#[from] idb::Error),
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
    #[error("no value found for key {0}")]
    NoValue(String),
    #[error("no value found for unknown key")]
    NoValueNoKey,
}
