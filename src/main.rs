// Happens when leptos::components take arguements with lifetimes.
#![allow(clippy::needless_lifetimes)]

use leptos::*;
use router::*;

mod assets;
mod css;
mod error;
mod items;
mod lobby;
mod pc;
mod rand;
mod router;
mod settings;
mod state;
mod svg;
mod tables;
mod utils;
mod views;

pub fn main() {
    #[cfg(debug_assertions)]
    {
        console_log::init_with_level(log::Level::Debug).expect("Error initialising logger");
        console_error_panic_hook::set_once();
    }

    mount_to_body(|cx| {
        view! { cx, <RouterScout /> }
    });
}
