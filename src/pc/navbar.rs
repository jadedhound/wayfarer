use leptos::*;
use leptos_router::*;

use crate::svg;
use crate::views::modal::modal_grey_screen;
use crate::views::revealer::RevealerScreen;
use crate::views::toast::toast_notification;

pub fn pc_navbar() -> impl IntoView {
    view! {
        <div class="fixed flex w-full justify-between p-4 bg-black z-[5]">
            <A href= "followers">
                <div class= "w-10 svg" inner_html=svg::FOLLOWERS />
            </A>
            <A href= "journal">
                <div class= "w-10 svg" inner_html=svg::PAPER_AND_QUILL />
            </A>
            <A href= "">
                <div class= "rounded-full p-2 border border-yellow-500">
                    <div class= "w-10 svg" inner_html=svg::CROWN />
                </div>
            </A>
            <A href= "inventory">
                <div class= "w-10 svg" inner_html=svg::BACKPACK />
            </A>
            <A href= "craft">
                <div class= "w-10 svg" inner_html=svg::ANVIL />
            </A>
        </div>
        <div class= "psuedo h-24" />
        <RevealerScreen />
        { modal_grey_screen() }
        { toast_notification() }
        <Outlet />
    }
}
