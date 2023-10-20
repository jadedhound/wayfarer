use leptos::*;

use super::view::ClassState;
use crate::buffs::Buff;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::views::modal::{ModalCenter, ModalState};

pub(super) fn optional_buff(i: usize) -> impl IntoView {
    let (pc, state) = (PC::expect(), ClassState::expect());
    let disabled = create_read_slice(state, move |state| state.exp.level().get() < i * 2 + 2);
    let info_text = move || {
        let no_buff = view! {
            <div class= "font-tight text-sky-500"> "CHOOSE CLASS BUFF" </div>
        }
        .into_view();
        state.with(|x| {
            x.optional[i]
                .as_ref()
                .map(|buff| buff.into_view())
                .unwrap_or(no_buff)
        })
    };
    let remove_current_buff = move || {
        state.update(|x| {
            x.chg_optional = i;
            if let Some(buff) = x.optional[i].take() {
                pc.update(|pc| {
                    if let Some(pos) = pc.buffs.position(|x| x.name == buff.name) {
                        pc.buffs.remove(pos);
                    }
                })
            }
        })
    };
    let open_buff_picker = move || {
        remove_current_buff();
        ModalState::show(10);
    };
    // When the button becomes disabled, remove the buff.
    create_effect(move |_| {
        if disabled.get() {
            remove_current_buff()
        }
    });

    view! {
        <button class= "col-start-2 col-span-6 btn-no-font bg-surface p-2"
            on:click=move |_| open_buff_picker()
            disabled=disabled
        >
            { info_text }
        </button>
    }
}

pub(super) fn buff_picker() -> impl IntoView {
    let (pc, state) = (PC::expect(), ClassState::expect());
    let class = pc.with_untracked(|pc| pc.class.0);
    let optional_changed =
        create_read_slice(state, |state| state.optional.iter().flatten().count());
    let class_buffs = move || {
        let _ = optional_changed.get();
        state.with_untracked(|state| {
            let has_buffs: Vec<&Buff> = state.optional.iter().flatten().collect();
            class
                .optional_buffs
                .iter()
                .map(|x| Buff::from(**x))
                .filter(|buff| !has_buffs.contains(&buff))
                .map(modal_buff_view)
                .collect_view()
        })
    };

    view! {
        <ModalCenter id=10>
            <h4 class= "text-center"> "Choose Buff" </h4>
            { class_buffs }
        </ModalCenter>
    }
}

fn modal_buff_view(buff: Buff) -> impl IntoView {
    let (pc, state) = (PC::expect(), ClassState::expect());
    let on_click = move |buff: &Buff| {
        state.update(|x| {
            x.optional[x.chg_optional] = Some(buff.clone());
        });
        pc.update(|pc| pc.buffs.add(buff.clone()));
        ModalState::hide()
    };
    let view = buff.into_view();

    view! {
        <button class= "btn-surface-no-font bg-zinc-700 p-2"
            on:click=move |_| on_click(&buff)
        >
            { view }
        </button>
    }
}
