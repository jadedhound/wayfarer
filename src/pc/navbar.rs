use leptos::*;
use leptos_router::*;

use crate::icons;
use crate::views::modal::modal_grey_screen;
use crate::views::revealer::revealer_screen;
use crate::views::toast::toast_notif;

pub fn pc_navbar() -> impl IntoView {
    let selected = create_memo(|_| {
        use_location().pathname.with(|path| {
            let last_word = path.split('/').last();
            match last_word {
                Some("realm") => 0,
                Some("journal") => 1,
                Some("overview") => 2,
                Some("inventory") => 3,
                _ => 4,
            }
        })
    });
    let icon_css = move |id: i32| {
        if selected.get() == id {
            "w-10 fill-yellow-400"
        } else {
            "w-10"
        }
    };

    view! {
        <Outlet />
        <div class= "psuedo h-24" />
        // Needs to be translated down to cover gap between bottom and div.
        <nav class= "fixed bottom-0 flex justify-around items-center w-full bg-zinc-950 z-[5]
                    border-t border-amber-600 px-8 h-16 pb-1 translate-y-1 fill-zinc-400">
            <A href= "realm">
                <div class=move || icon_css(0) inner_html=icons::WREATH />
            </A>
            <A href= "journal">
                <div class=move || icon_css(1) inner_html=icons::SCROLL />
            </A>
            <A href= "overview">
                <div class=move || icon_css(2) inner_html=icons::HELM />
            </A>
            <A href= "inventory">
                <div class=move || icon_css(3) inner_html=icons::BACKPACK />
            </A>
        </nav>
        { revealer_screen }
        { modal_grey_screen }
        { toast_notif }
    }
}
