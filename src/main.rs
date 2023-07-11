#![allow(clippy::let_with_type_underscore)]
#![allow(clippy::redundant_closure)]
use leptos::*;
use router::*;

mod assets;
mod error;
mod roster;
mod router;
mod settings;
mod state;
mod utils;

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
