use serde::{Deserialize, Serialize};

mod pchar;
pub use pchar::*;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppState {}

#[derive(Serialize, Deserialize)]
pub struct SessionState {}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCState(pub Vec<PChar>);
