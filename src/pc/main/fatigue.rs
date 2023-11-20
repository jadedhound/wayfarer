use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

pub fn fatigue() -> impl IntoView {
    let pc = PC::expect();
    let fatigue = PC::slice(|pc| 10 - pc.backpack.max_size());
    let is_locked = RwSignal::new(true);
    #[rustfmt::skip]
    let lock_icon = move || {
        if is_locked.get() { icons::LOCKED } else { icons::UNLOCKED }
    };
    #[rustfmt::skip]
    let lock_colour = move || {
        if is_locked.get() { "fill-zinc-500" } else { "fill-yellow-500" }
    };
    let decr = move |_| pc.update(|pc| pc.backpack.resize(pc.backpack.max_size() + 1));
    let incr = move |_| pc.update(|pc| pc.backpack.resize(pc.backpack.max_size() - 1));

    view! {
        <div class= "flex gap-4 border-y-2 border-orange-600 py-2">
            <button
                class=move || format!("ml-2 w-5 {}", lock_colour())
                on:click=move |_| is_locked.update(|x| *x = !*x)
                inner_html=lock_icon
            />
            <div class= "w-12 grow">
                <h6> "Fatigue" </h6>
                <div class= "italic">
                    "Each point of fatigue reduces the available inventory."
                </div>
            </div>
            <button
                class= "w-5 rotate-180 disabled:invisible"
                on:click=decr
                disabled=move || { fatigue.get() < 1 || is_locked.get() }
                inner_html=icons::RIGHT_CHEV
            />
            <h5 class= "self-center"> { fatigue } </h5>
            <button
                class= "w-5 disabled:invisible"
                on:click=incr
                disabled=move || { fatigue.get() > 9 || is_locked.get() }
                inner_html=icons::RIGHT_CHEV
            />
        </div>
    }
}
