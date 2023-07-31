use leptos::*;
mod item;
pub mod modal;
pub mod revealer;
pub mod toast;

pub use item::*;

use crate::utils::flat_concat;

fn format_funds(f: u32) -> String {
    let above_zero = |val, s| {
        if val > 0 {
            Some(format!("{val}{s}"))
        } else {
            None
        }
    };
    let mut f = f;
    let c = f % 10;
    f /= 10;
    let s = f % 100;
    f /= 100;
    let total = vec![
        above_zero(f, "gp"),
        above_zero(s, "sp"),
        above_zero(c, "cp"),
    ];
    flat_concat(total, " ").unwrap_or("0cp".to_string())
}

#[component]
pub fn Funds<F>(cx: Scope, sup: F) -> impl IntoView
where
    F: Fn() -> u32 + 'static,
{
    view! {
        cx,
        <div> { move || format_funds(sup()) } </div>
    }
}
