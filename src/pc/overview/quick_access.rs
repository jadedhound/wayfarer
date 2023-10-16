use leptos::*;

use crate::icons;
use crate::items::{Item, ItemProp};
use crate::pc::PC;
use crate::tables::spell_failure::gen_spell_failure;
use crate::utils::{expect_rw, RwProvided};
use crate::views::modal::{ModalCenter, ModalCenterCustom, ModalState};

#[derive(Default)]
struct SpellState {
    target: &'static str,
    effect: &'static str,
}

pub(super) fn quick_access() -> impl IntoView {
    provide_context(RwSignal::new(SpellState::default()));
    let no_items = PC::slice(|pc| pc.quick_access.iter().flatten().next().is_none());
    let table_or_help = move || {
        if no_items.get() {
            view! {
                <div class= "text-center italic p-2">
                    "Items in quick access slots will be shown here."
                </div>
            }
            .into_view()
        } else {
            table().into_view()
        }
    };
    view! {
        { table_or_help }
        { modal_spell_fail }
    }
}

fn table() -> impl IntoView {
    let quick_access = move || {
        PC::with(|pc| {
            pc.quick_access
                .iter()
                .enumerate()
                .flat_map(|(id, item)| {
                    let item = item.as_ref()?;
                    Some(quick_item(id, item))
                })
                .collect::<Vec<_>>()
        })
    };

    view! {
        <div class= "flex flex-col shaded-table">
            { quick_access }
        </div>
    }
}

fn quick_item(id: usize, item: &Item) -> impl IntoView {
    let use_btn = item
        .props
        .iter()
        .find(|prop| matches!(prop, ItemProp::Usable(_) | ItemProp::Buff(_)))
        .map(|_| use_item_btn(id));
    let spell_failure = item
        .props
        .iter()
        .find(|prop| matches!(prop, ItemProp::WildMagic(_)))
        .map(|_| spell_failure_btn(id));
    view! {
        <div class= "flex flex-col gap-1 p-2">
            { item.into_view() }
            { use_btn }
            { spell_failure }
        </div>
    }
}

fn use_item(id: usize) {
    PC::update(|pc| {
        let item = pc.quick_access[id].as_mut().unwrap();
        for prop in item.props.iter() {
            if let ItemProp::Buff(x) = prop {
                pc.buffs.add(x.clone())
            }
        }
        if let Some(count) = item.find_mut_counter() {
            count.curr -= 1;
            if count.is_zero() {
                pc.quick_access[id] = None
            }
        } else {
            pc.quick_access[id] = None
        }
    })
}

fn use_item_btn(id: usize) -> impl IntoView {
    let uses_left = move || {
        PC::with(|pc| {
            pc.quick_access[id]
                .as_ref()
                .and_then(|item| item.find_counter().map(|count| count.curr))
                .unwrap_or(1)
        })
    };

    view! {
        <div class= "flex gap-1">
            <div class= "border-2 border-green-600 rounded py-2 w-12 text-center font-tight">
                <div> { uses_left } </div>
            </div>
            <button
                class= "w-12 grow btn bg-green-800 py-2"
                on:click=move |_| use_item(id)
            >
                "USE"
            </button>
        </div>
    }
}
fn spell_failure_btn(id: usize) -> impl IntoView {
    let state = expect_rw::<SpellState>();
    let show_fail = move || {
        use_item(id);
        state.update(|x| {
            let result = gen_spell_failure();
            x.target = result.0;
            x.effect = result.1;
        });
        ModalState::open(0);
    };

    view! {
        <button
            class= "btn bg-red-800 py-2 flex justify-center gap-2"
            on:click=move |_| show_fail()
        >
            <div class= "w-6" inner_html=icons::WILD_BOLT />
            <div> "SPELL FAILURE" </div>
        </button>
    }
}

fn modal_spell_fail() -> impl IntoView {
    let state = expect_rw::<SpellState>();
    let result = move || {
        state.with(|x| {
            view! {
                <div class= "mb-2"> { x.target } </div>
                <div class= ""> { x.effect } </div>
            }
        })
    };

    view! {
        <ModalCenterCustom id=0>
            <div class= "relative w-full bg-surface shadow-md shadow-black rounded p-6">
                <div class= "absolute top-0 inset-x-0 flex justify-center -translate-y-10">
                    <div class= "bg-red-700 rounded-full p-5">
                        <div class= "w-10" inner_html=icons::WILD_BOLT />
                    </div>
                </div>
                <div class= "psuedo h-8" />
                <div class= "flex flex-col text-center">
                    { result }
                </div>
            </div>
        </ModalCenterCustom>
    }
}
