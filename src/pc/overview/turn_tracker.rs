use leptos::*;

use crate::svg;
use crate::{pc::PC, utils::RwProvided};

pub(super) fn turn_tracker() -> impl IntoView {
    let turns = move || PC::with(|pc| pc.turns).turns() % 6 + 1;
    let days = move || {
        let x = PC::with(|pc| pc.turns).days();
        format!("DAY {x}")
    };
    let change_turn = move |amount| PC::update(|pc| pc.turns.change_by(amount));

    view! {
        <div class= "italic text-sm text-center"> { days } </div>
        <div class= "flex-centered gap-4">
            <button
                class= "w-12 svg rotate-180"
                on:click=move |_| change_turn(-1)
                inner_html=svg::RIGHT_CHEV
            />
            <div class= "flex-centered flex-col rounded-full w-20 h-20 border-2 border-emerald-800">
                <h5 class= "border-b-2 border-emerald-800 w-12 text-center">
                    { turns }
                </h5>
                <h5> 6 </h5>
            </div>
            <button
                class= "w-12 svg"
                on:click=move |_| change_turn(1)
                inner_html=svg::RIGHT_CHEV
            />
        </div>
    }
}
