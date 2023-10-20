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

#[component]
pub fn FatalPage(err: Error) -> impl IntoView {
    view! {
        <div class= "flex-center flex-col gap-y-4 px-4 min-h-screen">
            <h1 class= "text-red-800"> "Fatal" </h1>
            <h5 class= "uppercase"> { err.to_string() } </h5>
        </div>
    }
}
