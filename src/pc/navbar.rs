use leptos::*;
use leptos_router::*;

use crate::icons;
use crate::views::revealer::{revealer_custom_screen, RevealerCustom};

pub fn pc_navbar() -> impl IntoView {
    let selected = create_memo(|_| {
        use_location().pathname.with(|path| {
            let last_word = path.split('/').last();
            match last_word {
                Some("realm") => 0,
                Some("followers") => 1,
                Some("main") => 2,
                Some("journal") => 3,
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
        <nav class= "fixed top-0 left-0 flex justify-between items-center w-full bg-zinc-950 z-[5] border-b border-amber-600 px-4 h-16 pb-1 fill-zinc-400">
            <A href= "realm">
                <div class=icon_css(0) inner_html=icons::CAMPFIRE />
            </A>
            <A href= "followers">
                <div class=icon_css(1) inner_html=icons::GROUP />
            </A>
            <A href= "overview" class= "rounded-full p-2 translate-y-8 border border-amber-600 bg-black">
                <div class=icon_css(2) inner_html=icons::HELM />
            </A>
            <A href= "journal">
                <div class=icon_css(3) inner_html=icons::BOOK />
            </A>
            { dropdown }
        </nav>
        // the `no-scroll` class added to body on modal popups, means that min-height is the only thing respected
        // and not just simply height.
        <div class= "psuedo min-h-[5rem]" />
        <Outlet />
        <div class= "psuedo min-h-[2rem]" />
    }
}

fn dropdown() -> impl IntoView {
    // Custom revealer because of z-index stacking context.
    let revealer = RevealerCustom::new();
    let rev_hidden = move || revealer.is_hidden();
    let colour = move || {
        if revealer.is_shown() {
            "w-10 stroke-yellow-400"
        } else {
            "w-10"
        }
    };

    view! {
        { revealer_custom_screen(revealer) }
        <div class= "relative inline-block">
            <button
                class= "flex items-center"
                on:click=move |_| revealer.show()
            >
                <div class=colour inner_html=icons::ELLIPSES />
            </button>
            <div
                class= "btn bg-surface flex flex-col z-40 w-28 absolute right-2 [&>*]:py-3"
                hidden=rev_hidden
            >
                <A class= "text-center text-red-500" href= "/"> "EXIT" </A>
            </div>
        </div>
    }
}
