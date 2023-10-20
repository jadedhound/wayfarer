use leptos::*;
use leptos_router::*;

use crate::icons;

#[component]
pub fn PCNavbar() -> impl IntoView {
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
        move || {
            if selected.get() == id {
                "w-10 fill-yellow-400"
            } else {
                "w-10"
            }
        }
    };

    view! {
        // Needs to be translated down to cover gap between bottom and div.
        <nav class= "fixed top-0 left-0 flex justify-around items-center w-full bg-zinc-950 z-[5]
                    border-b border-amber-600 px-8 h-16 pb-1 fill-zinc-400">
            <A href= "realm">
                <div class=icon_css(0) inner_html=icons::CAMPFIRE />
            </A>
            <A href= "journal">
                <div class=icon_css(1) inner_html=icons::BOOK />
            </A>
            <A href= "overview">
                <div class=icon_css(2) inner_html=icons::HELM />
            </A>
            <A href= "inventory">
                <div class=icon_css(3) inner_html=icons::BACKPACK />
            </A>
        </nav>
        <div class= "psuedo h-14" />
        <Outlet />
    }
}
