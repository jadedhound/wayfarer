use leptos::*;

use crate::buffs::{Buff, BuffProp};
use crate::icons;
use crate::pc::combat::use_button::use_button;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::views::delete_confirm::DeleteModal;

pub fn view((id, buff): (usize, &Buff)) -> impl IntoView {
    let pc = PC::expect();
    let show_delete_modal = move |_| DeleteModal::show(id);
    let counter = PC::slice(move |pc| {
        pc.buffs
            .get(id)
            .and_then(|x| {
                x.props.iter().find_map(|x| match x {
                    BuffProp::Count(count) => Some(*count),
                    _ => None,
                })
            })
            .unwrap_or_default()
    });
    let can_use = counter.get().max > 0;
    let use_item = move || {
        can_use.then(move || {
            use_button(
                move || counter.get(),
                move || pc.update(|pc| use_buff(pc, id)),
                false,
            )
        })
    };

    view! {
        <div class= "p-2">
            <div class= "flex gap-3">
                <button on:click=show_delete_modal>
                    <div class= "w-5 fill-red-600" inner_html=icons::TRASH />
                </button>
                <div class= "mb-2 w-12 grow"> { buff.into_view() } </div>
            </div>
            { use_item }
        </div>
    }
}

fn use_buff(pc: &mut PC, id: usize) {
    let counter = pc.buffs.get_mut(id).and_then(|buff| {
        buff.props.iter_mut().find_map(|x| match x {
            BuffProp::Count(count) => Some(count),
            _ => None,
        })
    });
    if let Some(counter) = counter {
        counter.curr = counter.curr.saturating_sub(1)
    }
}
