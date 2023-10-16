use leptos::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter};

use self::followers::followers;
use self::rest::rest;
use self::shop_grid::shops;
use crate::utils::enum_array::{EnumArray, EnumRef};

mod followers;
mod rest;
pub mod shop;
mod shop_grid;

pub fn realm() -> impl IntoView {
    view! {
        <h4 class= "text-center"> "Followers" </h4>
        { followers }
        <h4 class= "text-center"> "Shops" </h4>
        { shops }
        <h4 class= "text-center"> "Rest" </h4>
        { rest }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Follower {
    pub name: String,
    pub level: u8,
    pub stats: FolStatArray,
}

const FOL_STAT_COUNT: usize = FolStat::COUNT;

#[derive(Serialize, Deserialize, Clone, Copy, EnumCount, EnumIter, Display)]
pub enum FolStat {
    Health,
    Expertise,
    Mule,
    Morale,
}

impl EnumRef for FolStat {
    fn index(&self) -> usize {
        *self as usize
    }
}

pub type FolStatArray = EnumArray<FolStat, FOL_STAT_COUNT>;
