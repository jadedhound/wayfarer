use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

pub fn turn_tracker() -> impl IntoView {
    fn change_turn(pc: &mut PC, f: impl Fn(u64) -> u64) {
        pc.turns.0 = f(pc.turns.0)
    }
    let pc = PC::expect();
    let curr_turn = PC::slice(|pc| pc.turns.0 % 6 + 1);
    let no_back = PC::slice(|pc| pc.turns.0 == 0);

    view! {
        <h4 class= "text-center"> "Turn Tracker" </h4>
        <div class= "flex-center gap-2">
            <button
                class= "w-8 icons rotate-180 disabled:invisible"
                on:click=move |_| pc.update(|pc| pc.turns.sub(1))
                disabled=no_back
                inner_html=icons::RIGHT_CHEV
            />
            <div class= "flex-center flex-col rounded-full w-16 h-16 border-2 border-green-500">
                <h6 class= "border-b border-green-500 w-8 text-center">
                    { curr_turn }
                </h6>
                <h6> 6 </h6>
            </div>
            <button
                class= "w-8 icons disabled:invisible"
                on:click=move |_| pc.update(|pc| pc.turns.add(1))
                inner_html=icons::RIGHT_CHEV
            />
        </div>
    }
}
