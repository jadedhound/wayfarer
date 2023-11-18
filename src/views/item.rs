use leptos::*;

use crate::items::{damage_die, Item, ItemProp};
use crate::utils::add_operator;

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
        ItemProp::Resist => newline("Creatures can resist this effect"),
        ItemProp::Usable(x) => emphasised("use consumable", "text-sky-500", x),
        ItemProp::Range(x) => newline(format!("Range {x} ft")),
        ItemProp::Effect(x) => newline(x),
        ItemProp::Damage(x) => newline(format!("Deals {} damage", damage_die(*x))),
        ItemProp::Bulky => newline("Requires 2 inventory slots"),
        ItemProp::Concentration => newline("Requires concentration to maintain"),
        ItemProp::Score(abi, score) => newline(format!("{abi} {}", add_operator(*score))),
        ItemProp::Passive => emphasised(
            "passive",
            "text-orange-500",
            "can be used without being held",
        ),
        _ => None,
    }
}

fn newline<S>(line: S) -> Option<View>
where
    S: std::fmt::Display,
{
    let newline = view! {
        <div class= "capitalise"> { format!("{line}.") } </div>
    };
    Some(newline.into_view())
}

fn emphasised<S>(title: S, colour: S, line: S) -> Option<View>
where
    S: std::fmt::Display,
{
    let newline = view! {
        <div class= "capitalise">
            <span class=colour.to_string()> { format!("{title}: ") } </span>
            { format!("{line}.") }
        </div>
    };
    Some(newline.into_view())
}
