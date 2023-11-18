mod error;
mod icons;
mod indexeddb;
mod items;
mod lobby;
mod pc;
mod rand;
mod router;
mod settings;
mod utils;
mod views;

pub fn main() {
    leptos::mount_to_body(router::main_router);
}
