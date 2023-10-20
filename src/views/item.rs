use leptos::*;

use crate::buffs::{Buff, BuffProp};
use crate::items::{damage_die, Item, ItemProp};

impl IntoView for &Item {
    fn into_view(self) -> View {
        let props = self.props.iter().filter_map(prop_views).collect_view();

        view! {
            <div class= "flex flex-col h-full justify-center text-start">
                <div class= "uppercase font-tight"> { &self.name } </div>
                <div class= "italic" hidden=self.desc.is_empty()>
                    { &self.desc }
                </div>
                { props }
            </div>
        }
        .into_view()
    }
}

fn prop_views(prop: &ItemProp) -> Option<View> {
    match prop {
        ItemProp::Resist => newline("Creatures can resist this effect"),
        ItemProp::Usable(x) => newline(x),
        ItemProp::Food => newline("Restores health when consumed during rest"),
        ItemProp::Range(x) => newline(format!("Range {x} ft")),
        ItemProp::Effect(x) => newline(x),
        ItemProp::Damage(x) => newline(format!("Deals {} damage", damage_die(*x))),
        ItemProp::Buff(x) => Some(buff_desc(x)),
        ItemProp::Bulky => newline("Requires 2 hands to hold"),
        ItemProp::Concentration => newline("Requires concentration to maintain"),
        _ => None,
    }
}

fn newline<S>(s: S) -> Option<View>
where
    S: std::fmt::Display,
{
    let newline = view! {
        <div class= "capitalise"> { format!("{s}.") } </div>
    };
    Some(newline.into_view())
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
        <div class= "relative">
            <div class= "absolute right-0 inset-y-0 flex flex-col justify-center translate-x-4">
                <div class= "text-yellow-600 -rotate-90"> "BUFF" </div>
            </div>
            <div class= "border-y-2 border-yellow-600 my-1 pr-3">
                { buff.into_view() }
                { duration }
            </div>
        </div>
    }
    .into_view()
}
