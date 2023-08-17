use leptos::*;
use router::*;

mod assets;
mod error;
mod items;
mod lobby;
mod pc;
mod rand;
mod router;
mod svg;
mod tables;
mod utils;
mod views;

pub fn main() {
    #[cfg(debug_assertions)]
    {
        console_log::init_with_level(log::Level::Debug).expect("error initialising logger");
        console_error_panic_hook::set_once();
    }

    mount_to_body(main_router);
}
