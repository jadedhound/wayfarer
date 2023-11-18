use leptos::*;
use leptos_router::A;

use super::PC;

mod rally;
mod rest;
pub mod sell;
pub mod shop;

pub fn realm() -> impl IntoView {
    view! {
        <h4 class= "text-center"> "Town" </h4>
        <div class= "grid grid-cols-2 gap-1">
            { shop::list::shop_list }
            <A href= "sell" class= "btn bg-surface col-span-2 text-orange-500 text-center">
                "SELL ITEMS"
            </A>
        </div>
        <h4 class= "text-center"> "Rally" </h4>
        { rally::rally }
        <h4 class= "text-center"> "Rest" </h4>
        { rest::rest }
    }
}
