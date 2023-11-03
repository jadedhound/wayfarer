use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;

pub mod edit_note;
mod note_view;
pub mod overview;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Journal(IndexMap<Note>);

impl RwUtils for Journal {}

impl Deref for Journal {
    type Target = IndexMap<Note>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Journal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Note {
    name: String,
    body: String,
}

impl Note {
    fn new(name: String) -> Self {
        Self {
            name,
            body: String::new(),
        }
    }
}
