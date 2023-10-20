use leptos::*;

use crate::buffs::{Buff, BuffProp};
use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;

impl IntoView for &Buff {
    fn into_view(self) -> leptos::View {
        view! {
            { name(self) }
            { effect(self) }
            { uses_txt(self) }
        }
        .into_view()
    }
}

fn effect(buff: &Buff) -> impl IntoView {
    let effects: Vec<_> = buff
        .props
        .iter()
        .filter_map(|prop| match prop {
            BuffProp::Effect(x) => Some(format!("{x}.")),
            BuffProp::StatOverride(stat, by) => Some(format!("{stat} is now {by}.")),
            _ => None,
        })
        .collect();
    let effects = effects.join(" ");
    (!effects.is_empty()).then(move || {
        view! {
            <div class= "capitalise"> { effects } </div>
        }
    })
}

fn uses_txt(buff: &Buff) -> impl IntoView {
    let mut count = 0;
    let mut is_rally = None;
    for prop in buff.props.iter() {
        match prop {
            BuffProp::Rally => is_rally = Some(true),
            BuffProp::Rest => is_rally = Some(false),
            BuffProp::Count(x) => count = x.max,
            _ => (),
        }
    }
    is_rally.map(|is_rally| {
        const LONG_OR_SHORT: [&str; 2] = ["rest", "rally"];
        const USES: [&str; 2] = ["use", "uses"];
        let desc = format!(
            "{count} {} per {}.",
            USES[(count > 1) as usize],
            LONG_OR_SHORT[is_rally as usize]
        );
        view! {
            <div> { desc } </div>
        }
    })
}

fn name(buff: &Buff) -> impl IntoView {
    let pc = PC::expect();
    let mut colour = "";
    let mut turns = None;
    for prop in buff.props.iter() {
        match prop {
            BuffProp::Class => colour = "text-yellow-500",
            BuffProp::Rechargable => colour = "text-sky-500",
            BuffProp::Debuff => colour = "text-red-500",
            BuffProp::Expiry(x) => pc.with(|pc| turns = Some(pc.turns.abs_diff(*x))),
            _ => (),
        }
    }
    let turns_view = turns.map(|x| {
        view! {
            <div class= "rounded bg-red-800 flex items-center gap-2 px-2 font-tight">
                <div class= "w-4" inner_html=icons::CLOCK />
                <div class= "mt-px"> { x.to_string() } </div>
            </div>
        }
    });
    view! {
        <div class= "flex items-center">
            <div class=format!("w-12 grow font-tight uppercase line-clamp-2 {colour}")> { &buff.name } </div>
            { turns_view }
        </div>
    }
}
