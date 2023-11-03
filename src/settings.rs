use leptos::*;
use leptos_router::A;

use crate::icons;
use crate::settings::database::database;
use crate::utils::rw_utils::RwUtils;

mod database;

#[derive(Clone, Copy, Default)]
struct State {
    db_size: u64,
}

pub fn settings() -> impl IntoView {
    State::provide();

    view! {
        <div class= "fixed top-0 left-0 flex justify-between items-center bg-black border-b border-amber-600 h-16 px-4 w-full">
            <h3> "Settings" </h3>
            <A href= "/">
                <div class= "w-7" inner_html=icons::HOME />
            </A>
        </div>
        <div class= "psuedo h-16" />
        <h4 class= "text-center"> "Database" </h4>
        { database }
    }
}

impl RwUtils for State {}
