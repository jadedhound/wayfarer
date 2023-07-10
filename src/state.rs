use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppState {}

#[derive(Serialize, Deserialize)]
pub struct SessionState {}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCState(pub Vec<PClass>);

#[derive(Serialize, Deserialize, Clone)]
pub struct PClass {
    pub name: String,
}
