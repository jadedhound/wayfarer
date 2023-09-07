use leptos::*;

use crate::buffs::{Buff, BuffProp};
use crate::items::weapons::damage_die;
use crate::items::{Item, ItemProp};

impl IntoView for &Item {
    fn into_view(self) -> View {
        let props = self.props.iter().filter_map(prop_views).collect_view();

        view! {
            <div class= "flex flex-col h-full justify-center text-start">
                <div class= "uppercase font-tight"> { &self.name } </div>
                { props }
            </div>
        }
        .into_view()
    }
}

fn prop_views(prop: &ItemProp) -> Option<View> {
    match prop {
        ItemProp::Usable(x) => Some(newline(format!("Use: {x}."))),
        ItemProp::Edible(x) => Some(newline(format!("Heals {x} health."))),
        ItemProp::Spellbook(x) => Some(newline(format!("{x}."))),
        ItemProp::Range(x) => Some(newline(format!("Range {x} ft."))),
        ItemProp::Effect(x) => Some(newline(format!("{x}."))),
        ItemProp::Damage(x) => Some(newline(format!("Deals {} damage.", damage_die(*x)))),
        ItemProp::Buff(x) => Some(buff_desc(x)),
        _ => None,
    }
}

fn newline(s: String) -> View {
    view! { <div class= "capitalise"> { s } </div> }.into_view()
}

fn buff_desc(buff: &Buff) -> View {
    let duration = buff
        .props
        .iter()
        .find_map(|prop| match prop {
            BuffProp::Duration(x) => Some(x),
            _ => None,
        })
        .map(|turns| {
            view! {
                <div class= ""> { format!("Lasts for {turns}.") } </div>
            }
        });
    view! {
        <div> "Use: Applies the following buff." </div>
        <div class= "border-y-2 border-yellow-600 my-1">
            { buff.into_view() }
            { duration }
        </div>
    }
    .into_view()
}
