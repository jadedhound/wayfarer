use leptos::*;

use crate::icons;
use crate::pc::update;
use crate::views::revealer::Revealer;

#[component]
pub fn Rest() -> impl IntoView {
    let days = RwSignal::new(1);
    let steps = (1..=7).map(|x| view! { <div> { x } </div> }).collect_view();
    let change_days = move |ev: String| {
        let num = ev.parse::<u64>().unwrap_or(1);
        days.set(num);
    };

    view! {
        <div class= "italic text-center">
            "Every rest removes fatigue but only safe rests restore health."
        </div>
        <div class= "relative pointer-events-none h-10">
            <div class= "absolute grid grid-cols-7 text-center items-center w-full h-full font-tight">
                { steps }
            </div>
            <input
                class= "range sky-bar bg-sky-950 h-full w-full pointer-events-auto"
                type= "range"
                min=0
                max=7
                step=1
                value=1
                on:input=move |ev| change_days(event_target_value(&ev))
            />
        </div>
        <RestButton days />
    }
}

#[component]
fn RestButton(days: RwSignal<u64>) -> impl IntoView {
    let is_safe = RwSignal::new(false);
    let days_view = move || {
        let safe = is_safe.get().then_some("SAFE ").unwrap_or_default();
        let days = days.get();
        let days_txt = if days == 1 { "DAY" } else { "DAYS" };
        format!("{safe}REST {days} {days_txt}")
    };
    let complete_rest = move || {
        update::on_rest(days.get(), is_safe.get());
        is_safe.set(false);
        Revealer::hide();
    };

    view! {
        <div class= "flex gap-2">
            <div class= "relative">
                <input
                    class= "absolute opacity-0 w-full h-full"
                    type= "checkbox"
                    on:click=move |_| is_safe.update(|x| *x = !*x)
                    prop:checked=move || is_safe.get()
                />
                <div class= "checkmark-btn flex-center p-2 h-full">
                    <div class= "w-6" inner_html=icons::HOME />
                </div>
            </div>
            <div class= "relative w-12 grow">
                <button
                    class= "btn bg-surface py-2 w-full"
                    on:click= move |_| Revealer::show('r', 0)
                    disabled=move || days.get() < 1
                >
                    { days_view }
                </button>
                <div hidden=move || !Revealer::is_shown('r', 0)>
                    <button
                        class= "absolute top-0 h-full w-full btn bg-blue-800 z-40"
                        on:click= move |_| complete_rest()
                    >
                        "CONFIRM"
                    </button>
                </div>
            </div>
        </div>
    }
}
