use leptos::*;
use leptos_router::A;

use super::PC;
use crate::buffs::BuffProp;

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

fn restore_buffs<F>(pc: &mut PC, filter: F)
where
    F: FnMut(&BuffProp) -> bool + Copy,
{
    let buff_arr = pc
        .buffs
        .values_mut()
        .filter(|buff| buff.props.iter().any(filter));
    for buff in buff_arr {
        let count = buff.props.iter_mut().find_map(|prop| match prop {
            BuffProp::Count(count) => Some(count),
            _ => None,
        });
        if let Some(count) = count {
            count.curr = count.max
        }
    }
}
