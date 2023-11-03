use leptos::*;
use web_sys::Event;

use super::view_optional::{buff_picker, optional_buff};
use crate::buffs::{Buff, BuffProp};
use crate::icons;
use crate::pc::session::Session;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

pub(super) struct ClassState {
    pub optional: [Option<Buff>; 3],
    pub chg_optional: usize,
}

impl RwUtils for ClassState {}

impl Default for ClassState {
    fn default() -> Self {
        PC::expect().with(|pc| Self {
            optional: filter_optional_buffs(pc),
            chg_optional: 0,
        })
    }
}

pub fn class() -> impl IntoView {
    let (pc, sesh) = (PC::expect(), Session::expect());
    ClassState::provide();
    let name = move || {
        let class = pc.with(|pc| pc.class.0);
        let lvl = sesh.with(|sesh| sesh.level.get());
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
            { format!("Every level increases your base health by 1 and your guard by {guard_bonus}.") }
        </div>
    }
}

fn level() -> impl IntoView {
    let (pc, sesh) = (PC::expect(), Session::expect());
    let level = move || sesh.with(|sesh| sesh.level);
    let curr_exp = move || pc.with(|pc| pc.class.1.get());
    let exp_needed = PC::slice(move |pc| level().max_exp().saturating_sub(pc.class.1.get()));
    let usr_num = RwSignal::new(0);
    let change_exp = |op: isize| move |_| pc.update(|pc| pc.class.1.change(usr_num.get() * op));
    let on_usr_num = move |ev: Event| {
        let num = event_target_value(&ev).parse::<isize>().unwrap_or_default();
        usr_num.set(num)
    };
    let min = move || level().min_exp();
    let max = move || level().max_exp();

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
                class= "btn bg-red-800"
                on:click=change_exp(-1)
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
                class= "btn bg-green-800"
                on:click=change_exp(1)
            >
                <div class= "w-4" inner_html=icons::PLUS />
            </button>
        </div>
    }
}

fn base_buff(i: usize) -> impl IntoView {
    let pc = PC::expect();
    let disabled = Session::slice(move |sesh| sesh.level.get() < i * 2 + 1);
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
            class= "col-start-2 col-span-6 btn !font-[inherit] bg-surface [&:disabled>*>*]:text-zinc-500"
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
