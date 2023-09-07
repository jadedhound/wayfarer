use leptos::*;

use super::pc_class::PCClassRef;
use crate::buffs::{Buff, BuffProp};
use crate::pc::class_view::optional::{buff_picker, optional_buff};
use crate::pc::PC;
use crate::utils::{expect_rw, RwProvided};

mod optional;

struct ClassState {
    level: u8,
    class: PCClassRef,
    optional: [Option<Buff>; 3],
    changing_opt: usize,
}

pub fn class() -> impl IntoView {
    let state = PC::with(|pc| {
        create_rw_signal(ClassState {
            level: pc.class.1,
            class: pc.class.0,
            optional: filter_optional_buffs(pc),
            changing_opt: 0,
        })
    });
    let class_name = state.with(|x| x.class.as_ref().to_string());
    provide_context(state);

    view! {
        <div class= "grid grid-cols-8 px-2 gap-y-2">
            <h2 class= "col-span-8 mb-5"> { class_name } </h2>
            { level }
            { base_buff(0) }
            { optional_buff(0) }
            { base_buff(1) }
            { optional_buff(1) }
            { base_buff(2) }
            { optional_buff(2) }
        </div>
        { buff_picker }
    }
}

fn level() -> impl IntoView {
    let state = expect_rw::<ClassState>();
    let level = move || state.with(|x| format!("Level {}", x.level));
    let cant_decr = move || state.with(|x| x.level < 2);
    let chg_level = move |by: i16| {
        let new_level = (state.with(|x| x.level as i16) + by) as u8;
        state.update(|x| x.level = new_level);
        PC::update(|pc| pc.class.1 = new_level);
    };
    let cant_incr = move || state.with(|x| x.level > 5);

    view! {
        <button
            class= "col-start-2 text-center btn text-3xl bg-red-800 pb-2"
            on:click=move |_| chg_level(-1)
            disabled=cant_decr
            inner_html= "&laquo;"
        />
        <h4 class= "col-span-4 text-center self-center"> { level } </h4>
        <button
            class= "text-center btn text-3xl bg-green-800 pb-2"
            on:click=move |_| chg_level(1)
            disabled=cant_incr
            inner_html= "&raquo;"
        />
    }
}

fn base_buff(i: usize) -> impl IntoView {
    let state = expect_rw::<ClassState>();
    let disabled = create_read_slice(state, move |x| x.level < i as u8 * 2 + 1);
    let buff = state.with(|x| Buff::from(*x.class.base_buffs[i]));
    let buff_view = buff.into_view();

    // Add or remove the buff depending on if the current level
    // enables or disables the button.
    create_effect(move |_| {
        let remove_buff = disabled.get();
        let buff = buff.clone();
        PC::update(|pc| {
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
        <button class= "col-start-2 col-span-6 btn bg-surface p-2"
            disabled=disabled
        >
            { buff_view }
        </button>
    }
}

fn filter_optional_buffs(pc: &PC) -> [Option<Buff>; 3] {
    let mut arr = [(); 3].map(|_| None);
    let class_base = pc.class.0.base_buffs.map(|x| x.name);
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
