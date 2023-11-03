use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::utils::turns::TURNS_IN_DAY;
use crate::views::checkbox::Checkbox;

struct State {
    locked: bool,
}

pub fn time_tracker() -> impl IntoView {
    State::provide();

    view! {
        { time_lock }
        <div class= "grid grid-cols-2 gap-2 font-tight text-center">
            <div class= ""> "DATE" </div>
            <div class= ""> "WATCH" </div>
            { date }
            { watches }
        </div>
        { turns }
    }
}

fn time_lock() -> impl IntoView {
    let state = State::expect();
    let is_locked = State::slice(|state| state.locked);
    let set_value = move || state.update(|state| state.locked = !state.locked);
    let text = move || {
        if is_locked.get() {
            ("TIME LOCKED", icons::LOCKED)
        } else {
            ("TIME UNLOCKED", icons::UNLOCKED)
        }
    };

    view! {
        <Checkbox
            checked=is_locked
            checked_colour= "fill-zinc-500 border-zinc-600 text-zinc-500"
            unchecked_colour= "fill-yellow-500 border-yellow-600 text-yellow-500"
            on_click=set_value
        >
            <div class= "flex gap-2 justify-center">
                <div class= "w-5 ml-6" inner_html=move || text().1 />
                <div class= "w-32 text-left"> { move || text().0 } </div>
            </div>
        </Checkbox>

    }
}

fn change_time<F>(pc: &mut PC, new_time: F)
where
    F: Fn(u64) -> u64 + 'static,
{
    pc.turns.0 = new_time(pc.turns.0);
}

fn date() -> impl IntoView {
    fn change_day<F>(pc: &mut PC, new_day: F)
    where
        F: Fn(u64) -> u64 + 'static,
    {
        pc.turns.0 = new_day(pc.turns.in_days()) * TURNS_IN_DAY;
    }

    let (pc, state) = (PC::expect(), State::expect());
    // 0 = Day, 1 = Month, 2 = Year
    let date_format = RwSignal::new(0);
    let turns = PC::slice(|pc| pc.turns);
    let locked = move || state.with(|state| state.locked);
    let no_back = move || turns.get().in_days() < 1 || locked();
    let toggle_format = move |_| date_format.update(|i| *i = if *i == 2 { 0 } else { *i + 1 });
    let date = move || {
        let turns = turns.get().0;
        match date_format.get() {
            0 => format!("DAY {}", (turns / TURNS_IN_DAY) % 28),
            1 => format!("MONTH {}", (turns / (TURNS_IN_DAY * 28)) % 12),
            _ => format!("YEAR {}", turns / (TURNS_IN_DAY * 28 * 12)),
        }
    };

    view! {
        <div class= "flex-center gap-4">
            <button
                class= "w-5 rotate-180 disabled:invisible"
                on:click=move |_| pc.update(|pc| change_day(pc, |day| day - 1))
                disabled=no_back
                inner_html=icons::RIGHT_CHEV
            />
            <button
                class= "font-tight"
                on:click=toggle_format
            >
                { date }
            </button>
            <button
                class= "w-5 disabled:invisible"
                on:click=move |_| pc.update(|pc| change_day(pc, |day| day + 1))
                disabled=locked
                inner_html=icons::RIGHT_CHEV
            />
        </div>
    }
}

fn watches() -> impl IntoView {
    const WATCH: u64 = 6 * 6;
    let (pc, state) = (PC::expect(), State::expect());
    let locked = move || state.with(|state| state.locked);
    let watch_index = PC::slice(|pc| (pc.turns.0 % TURNS_IN_DAY) / WATCH);
    let no_back = move || watch_index.get() < 1 || locked();
    let no_forward = move || watch_index.get() > 2 || locked();
    let curr_watch = move || {
        let (colour, icon) = match watch_index.get() {
            0 => ("fill-red-400", icons::RISING_SUN),
            1 => ("fill-yellow-500", icons::SUN),
            2 => ("fill-orange-500", icons::SETTING_SUN),
            _ => ("fill-blue-800", icons::MOON),
        };
        view! {
            <div class=format!("w-8 {colour}") inner_html=icon />
        }
    };

    view! {
        <div class= "flex-center gap-4">
            <button
                class= "w-5 rotate-180 disabled:invisible"
                on:click=move |_| pc.update(|pc| change_time(pc, |time| time - WATCH))
                disabled=no_back
                inner_html=icons::RIGHT_CHEV
            />
            { curr_watch }
            <button
                class= "w-5 disabled:invisible"
                on:click=move |_| pc.update(|pc| change_time(pc, |time| time + WATCH))
                disabled=no_forward
                inner_html=icons::RIGHT_CHEV
            />
        </div>
    }
}

fn turns() -> impl IntoView {
    let (pc, state) = (PC::expect(), State::expect());
    let curr_turn = PC::slice(|pc| pc.turns.0 % 6 + 1);
    let change_turn = move |amount| {
        pc.update(|pc| pc.turns.change_by(amount));
        state.update(|state| state.locked = true);
    };
    let no_back = PC::slice(|pc| pc.turns.0 % TURNS_IN_DAY == 0);
    let no_forward = PC::slice(|pc| (pc.turns.0 + 1) % TURNS_IN_DAY == 0);

    view! {
        <div class= "text-center font-tight"> "TURN" </div>
        <div class= "flex-center gap-2">
            <button
                class= "w-8 icons rotate-180 disabled:invisible"
                on:click=move |_| change_turn(-1)
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
                on:click=move |_| change_turn(1)
                disabled=no_forward
                inner_html=icons::RIGHT_CHEV
            />
        </div>
    }
}

impl RwUtils for State {}

impl Default for State {
    fn default() -> Self {
        Self { locked: true }
    }
}
