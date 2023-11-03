use leptos::*;

use crate::views::revealer::{RevLocation, Revealer};

/// A button that askes for confirmation before executing the click.
#[component]
pub fn ConfirmButton<F>(
    children: Children,
    location: RevLocation,
    on_click: F,
    #[prop(optional)] disabled: Signal<bool>,
    #[prop(default = "btn bg-surface")] class: &'static str,
    #[prop(default = "btn bg-sky-800 ")] confirm_class: &'static str,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    let confirm_hidden = move || !Revealer::is_shown(location, 0);
    let exec_confirm = move |_| {
        Revealer::show(location, 0);
    };
    let execute_action = move |_| {
        Revealer::hide();
        on_click()
    };
    view! {
        <div class= "relative">
            <button
                class=format!("{class} w-full")
                disabled=disabled
                on:click=exec_confirm
            >
                { children() }
            </button>
            <button
                class=format!("{confirm_class} absolute top-0 left-0 w-full h-full z-[31]")
                hidden=confirm_hidden
                on:click=execute_action
            >
                "CONFIRM"
            </button>
        </div>
    }
}
