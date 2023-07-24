use std::clone;

use leptos::*;

pub const MODAL_MENU_BTN: &str = "rounded w-full py-1 shadow-sm shadow-zinc-900 bg-zinc-700";

#[component]
pub fn GreyScreen<H>(cx: Scope, children: Children, hidden: H) -> impl IntoView
where
    H: Fn() -> bool + 'static,
{
    view! {
        cx,
        <div class= "fixed inset-0" hidden=move || hidden()>
            <div class= "h-full bg-zinc-800 bg-opacity-75"/>
                { children(cx) }
        </div>
    }
}

#[component]
pub fn ModalMenu<H, D, S>(
    cx: Scope,
    children: Children,
    title: S,
    hidden: H,
    dismiss: D,
) -> impl IntoView
where
    H: Fn() -> bool + 'static,
    D: Fn() + 'static + Copy,
    S: std::fmt::Display,
{
    let title = title.to_string();
    view! {
        cx,
        <GreyScreen hidden=hidden>
            <div class= "absolute inset-0 w-full z-10 flex flex-col">
                <div class= "grow" on:click=move |_| dismiss() />
                <div class= "w-full px-2">
                    <div class= "bg-zinc-800 rounded shadow-sm shadow-zinc-900 h-full w-full text-center flex flex-col p-4 gap-2">
                        <h3> { title } </h3>
                        <div class= "psuedo h-4" />
                        { children(cx) }
                    </div>
                </div>
                <div class= "grow" on:click=move |_| dismiss() />
            </div>
        </GreyScreen>
    }
}
