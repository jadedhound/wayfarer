use leptos::*;
use serde::{Deserialize, Serialize};

use crate::indexeddb::dberror::DBError;

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Error {
    #[error("page not found")]
    NotFound,
    #[error("IndexedDB error: {0}")]
    SimpleIndex(String),
    #[error("ID not found in lobby list")]
    PCNotFound,
}

impl From<DBError> for Error {
    fn from(value: DBError) -> Self {
        Self::SimpleIndex(value.to_string())
    }
}

pub fn fatal_page(err: Error) -> impl IntoView {
    view! {
        <div class= "flex-center flex-col gap-y-4 px-4 min-h-screen text-center">
            <h1 class= "text-red-800"> "Fatal" </h1>
            <div class= "font-tight uppercase text-xl"> { err.to_string() } </div>
        </div>
    }
}
