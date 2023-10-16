use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::RwProvided;

pub(super) fn turn_tracker() -> impl IntoView {
    let turns = move || PC::with(|pc| pc.turns).in_turns() % 6 + 1;
    let days = create_memo(move |_| {
        let x = PC::with(|pc| pc.turns).in_days();
        format!("DAY {x}")
    });
    let change_turn = move |amount| PC::update(|pc| pc.turns.change_by(amount));
    let cannot_rewind = create_memo(move |_| {
        let mut turns = PC::with(|pc| pc.turns);
        let curr = turns.in_days();
        turns.change_by(-1);
        let next = turns.in_days();
        curr != next
    });

    view! {
        <div class= "italic text-sm text-center"> { days } </div>
        <div class= "flex-center gap-4">
            <button
                class= "w-12 icons rotate-180 disabled:fill-zinc-500"
                on:click=move |_| change_turn(-1)
                disabled=cannot_rewind
                inner_html=icons::RIGHT_CHEV
            />
            <div class= "flex-center flex-col rounded-full w-20 h-20 border-4 border-green-500">
                <h5 class= "border-b-2 border-green-500 w-12 text-center">
                    { turns }
                </h5>
                <h5> 6 </h5>
            </div>
            <button
                class= "w-12 icons"
                on:click=move |_| change_turn(1)
                inner_html=icons::RIGHT_CHEV
            />
        </div>
    }
}
