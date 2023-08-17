use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Error {
    #[error("page not found")]
    NotFound,
    #[error("IndexedDB error: {0}")]
    SimpleIndex(String),
    #[error("ID not found in lobby list")]
    PCNotFound,
}

impl From<simple_index::Error> for Error {
    fn from(value: simple_index::Error) -> Self {
        Self::SimpleIndex(value.to_string())
    }
}

pub fn fatal_pg(err: Error) -> impl IntoView {
    view! {
        <div class="h-32 grow flex-centered flex-col space-y-4 text-center px-4">
            <h1 class= "text-red-800"> "Fatal" </h1>
            <h5> { err.to_string().to_uppercase() } </h5>
        </div>
    }
}
