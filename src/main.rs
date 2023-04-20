#![allow(clippy::let_with_type_underscore)]
use leptos::*;
use router::*;

mod class;
mod errors;
mod home;
mod render_page;
mod router;
mod utils;

pub fn main() {
    #[cfg(debug_assertions)]
    {
        console_log::init_with_level(log::Level::Debug).expect("Error initialising logger");
        console_error_panic_hook::set_once();
    }

    mount_to_body(|cx| {
        view! { cx, <AppRouter /> }
    });
}
