use leptos::*;
use leptos_router::*;

use crate::svg;

#[component]
pub fn NavBarWithOutlet(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="fixed flex w-full justify-between p-4 bg-zinc-950">
            <A href= "/">
                <div class= "w-10" inner_html=svg::DUNGEON_DOOR />
            </A>
            <A href= "journal">
                <div class= "w-10" inner_html=svg::PAPER_AND_QUILL />
            </A>
            <A href= "">
                <div class= "rounded-full p-2 border border-yellow-500">
                    <div class= "w-10" inner_html=svg::CROWN />
                </div>
            </A>
            <A href= "inventory">
                <div class= "w-10" inner_html=svg::BACKPACK />
            </A>
            <A href= "crafting">
                <div class= "w-10" inner_html=svg::ANVIL />
            </A>
        </div>
        <div class= "psuedo h-24" />
        <Outlet />
    }
}
