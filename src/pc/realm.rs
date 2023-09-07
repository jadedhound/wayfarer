use leptos::*;
use serde::{Deserialize, Serialize};
use strum::{Display, FromRepr};

use self::followers::followers;
use self::rest::rest;
use self::shops::shops;

mod followers;
mod rest;
mod shops;

pub fn realm() -> impl IntoView {
    view! {
        <h3 class= "text-center"> "Followers" </h3>
        { followers }
        <h3 class= "text-center"> "Shops" </h3>
        { shops }
        <h3 class= "text-center"> "Rest" </h3>
        { rest }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Display, FromRepr, Default)]
pub enum Experience {
    #[default]
    Novice,
    Journeyman,
    Expert,
    Master,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Follower {
    pub name: String,
    pub exp: Experience,
}

impl Follower {
    pub fn inv_incr(&self) -> usize {
        2 * (self.exp as usize + 1)
    }
}
