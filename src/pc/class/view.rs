use leptos::*;
use web_sys::Event;

use super::level::ClassExp;
use super::view_optional::{buff_picker, optional_buff};
use crate::buffs::{Buff, BuffProp};
use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

pub(super) struct ClassState {
    pub exp: ClassExp,
    pub optional: [Option<Buff>; 3],
    pub chg_optional: usize,
}

impl RwUtils for ClassState {
    type Item = Self;
}

impl Default for ClassState {
    fn default() -> Self {
        PC::expect().with(|pc| Self {
            exp: pc.class.1,
            optional: filter_optional_buffs(pc),
            chg_optional: 0,
        })
    }
}

pub fn class() -> impl IntoView {
    let pc = PC::expect();
    let state = ClassState::provide();
    let class = pc.with(|x| x.class.0.as_ref().to_string());
    let name = move || {
        let lvl = state.with(|state| state.exp.level().get());
        format!("{class} {lvl}")
    };

    view! {
        <h4 class= "text-center"> { name } </h4>
        { level_up_info }
        { level }
        { base_buff(0) }
        { optional_buff(0) }
        { base_buff(1) }
        { optional_buff(1) }
        { base_buff(2) }
        { optional_buff(2) }
        { buff_picker }
    }
}

fn level_up_info() -> impl IntoView {
    let pc = PC::expect();
    let guard_bonus = pc.with_untracked(|pc| pc.class.0.guard_bonus);

    view! {
        <div class= "">
            <span class= "font-bold"> "Health and Guard. " </span>
            { format!("Every level increases you base health by 1 and your guard by {guard_bonus}.") }
        </div>
    }
}

fn level() -> impl IntoView {
    let (pc, state) = (PC::expect(), ClassState::expect());
    let level = create_read_slice(state, |state| state.exp.level());
    let curr_exp = move || state.with(|state| state.exp.get());
    let exp_needed = PC::slice(move |pc| level.get().max_exp().saturating_sub(pc.class.1.get()));
    let usr_num = RwSignal::new(0);
    let incr_exp = move |_| {
        pc.update(|pc| pc.class.1.incr(usr_num.get()));
        state.update(|state| state.exp = pc.with(|pc| pc.class.1));
    };
    let decr_exp = move |_| {
        pc.update(|pc| pc.class.1.decr(usr_num.get()));
        state.update(|state| state.exp = pc.with(|pc| pc.class.1));
    };
    let on_usr_num = move |ev: Event| {
        let num = event_target_value(&ev).parse::<usize>().unwrap_or_default();
        usr_num.set(num)
    };
    let min = move || level.get().min_exp();
    let max = move || level.get().max_exp();

    view! {
        <div class= "relative h-12">
            <input
                class= "range bg-yellow-950 yellow-bar h-full w-full pointer-events-none"
                type= "range"
                prop:min=min
                prop:max=max
                prop:value=move || pc.with(|pc| pc.class.1.get())
            />
            <div class= "absolute top-0 left-0 w-full h-full flex justify-between items-center font-tight px-3">
                <div class= ""> { min } </div>
                <div class= ""> { max } </div>
            </div>
        </div>
        <div class= "font-tight text-center">
            <div> { format!("CURRENT EXP: {}", curr_exp()) } </div>
            <div> { format!("EXP NEEDED: {}", exp_needed.get()) } </div>
        </div>
        <div class= "flex gap-2">
            <button
                class= "btn bg-red-800 px-3"
                on:click=decr_exp
            >
                <div class= "w-4" inner_html=icons::MINUS />
            </button>
            <input
                class= "input w-12 grow text-center"
                type= "number"
                prop:value=usr_num
                on:input=on_usr_num
            />
            <button
                class= "btn bg-green-800 px-3"
                on:click=incr_exp
            >
                <div class= "w-4" inner_html=icons::PLUS />
            </button>
        </div>
    }
}

fn base_buff(i: usize) -> impl IntoView {
    let (pc, state) = (PC::expect(), ClassState::expect());
    let disabled = create_read_slice(state, move |state| state.exp.level().get() < i * 2 + 1);
    let buff = pc.with(|pc| Buff::from(*pc.class.0.base_buffs[i]));
    let buff_view = buff.into_view();

    // Add or remove the buff depending on if the current level
    // enables or disables the button.
    create_effect(move |_| {
        let remove_buff = disabled.get();
        let buff = buff.clone();
        pc.update(|pc| {
            let buff_pos = pc.buffs.position(|x| x.name == buff.name);
            // Remove the buff.
            if remove_buff {
                if let Some(i) = buff_pos {
                    pc.buffs.remove(i);
                }
            // Only add the buff if it doesn't currently exist.
            } else if buff_pos.is_none() {
                pc.buffs.add(buff)
            }
        })
    });

    view! {
        <button
            class= "col-start-2 col-span-6 btn-no-font bg-surface p-2"
            disabled=disabled
        >
            { buff_view }
        </button>
    }
}

fn filter_optional_buffs(pc: &PC) -> [Option<Buff>; 3] {
    let mut arr = [(); 3].map(|_| None);
    let class_base: Vec<&str> = pc.class.0.base_buffs.iter().map(|x| x.name).collect();
    // Considered an optional buff if it has `BuffProp::Class`
    // and isn't one of the class base buffs.
    pc.buffs
        .values()
        .filter(|buff| buff.props.contains(&BuffProp::Class))
        .filter(|buff| !class_base.contains(&buff.name.as_str()))
        .enumerate()
        .for_each(|(i, buff)| {
            if i < arr.len() {
                arr[i] = Some(buff.clone())
            }
        });
    arr
}
