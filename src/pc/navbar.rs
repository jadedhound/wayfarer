use leptos::*;
use leptos_router::*;

use crate::icons;
use crate::views::modal::modal_grey_screen;
use crate::views::revealer::revealer_screen;
use crate::views::toast::toast_notif;

pub fn pc_navbar() -> impl IntoView {
    let selected = create_memo(|_| {
        use_location().pathname.with(|path| {
            let letter = path.chars().skip(4).skip_while(|x| x != &'/').nth(1);
            match letter {
                Some('r') => 0,
                Some('j') => 1,
                None => 2,
                Some('i') => 3,
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
        <div class= "fixed bottom-0 flex justify-around items-center w-full bg-zinc-950 z-[5]
                    border-t border-amber-600 px-8 h-16 pb-1 translate-y-1 fill-zinc-400">
            <A href= "realm">
                <div class=move || icon_css(0) inner_html=icons::WREATH />
            </A>
            <A href= "journal">
                <div class=move || icon_css(1) inner_html=icons::SCROLL />
            </A>
            <A href= "">
                <div class=move || icon_css(2) inner_html=icons::HELM />
            </A>
            <A href= "inventory">
                <div class=move || icon_css(3) inner_html=icons::BACKPACK />
            </A>
        </div>
        { revealer_screen }
        { modal_grey_screen }
        { toast_notif }
    }
}
