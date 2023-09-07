use leptos::*;

use crate::buffs::Buff;
use crate::pc::class_view::ClassState;
use crate::pc::PC;
use crate::utils::{expect_rw, RwProvided};
use crate::views::modal::{CenterModal, ModalState};

pub(super) fn optional_buff(i: usize) -> impl IntoView {
    let state = expect_rw::<ClassState>();
    let disabled = create_read_slice(state, move |x| x.level < i as u8 * 2 + 2);
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
            x.changing_opt = i;
            if let Some(buff) = x.optional[i].take() {
                PC::update(|pc| {
                    if let Some(pos) = pc.buffs.position(|x| x.name == buff.name) {
                        pc.buffs.remove(pos);
                    }
                })
            }
        })
    };
    let open_buff_picker = move || {
        remove_current_buff();
        ModalState::open(0);
    };
    // When the button becomes disabled, remove the buff.
    create_effect(move |_| {
        if disabled.get() {
            remove_current_buff()
        }
    });

    view! {
        <button class= "col-start-2 col-span-6 btn bg-surface p-2"
            on:click=move |_| open_buff_picker()
            disabled=disabled
        >
            { info_text }
        </button>
    }
}

pub(super) fn buff_picker() -> impl IntoView {
    let state = expect_rw::<ClassState>();
    let on_click = move |buff: &Buff| {
        state.update(|x| {
            x.optional[x.changing_opt] = Some(buff.clone());
        });
        PC::update(|pc| pc.buffs.add(buff.clone()));
        ModalState::dismiss()
    };
    let buff_view = move |buff: Buff| {
        let buff_clone = buff.clone();
        view! {
            <button class= "btn-surface bg-zinc-700 p-2"
                on:click=move |_| on_click(&buff)
            >
                { buff_clone.into_view() }
            </button>
        }
    };
    let class_buffs = move || {
        state.with(|x| {
            let has_buffs: Vec<&Buff> = x.optional.iter().flatten().collect();
            x.class
                .optional_buffs
                .iter()
                .map(|x| Buff::from(**x))
                .filter(|buff| !has_buffs.contains(&buff))
                .map(buff_view)
                .collect_view()
        })
    };

    view! {
        <CenterModal title=|| "CHOOSE BUFF" id=0>
            <div class= "flex flex-col gap-2">
                { class_buffs }
            </div>
        </CenterModal>
    }
}
