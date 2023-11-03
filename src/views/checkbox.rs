use leptos::*;

#[component]
pub fn Checkbox<F>(
    children: Children,
    checked: Signal<bool>,
    on_click: F,
    #[prop(default = "")] checked_colour: &'static str,
    #[prop(default = "border-zinc-700 fill-zinc-500 text-zinc-500")] unchecked_colour: &'static str,
    #[prop(optional)] class: &'static str,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    let colours = move || {
        if checked.get() {
            checked_colour
        } else {
            unchecked_colour
        }
    };

    view! {
        <button
            class=move || format!("border-2 rounded font-tight p-2 {class} {}", colours())
            on:click=move |_| on_click()
        >
            { children() }
        </button>
    }
}
