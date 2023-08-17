use leptos::*;

use crate::items::buffs::Buff;
use crate::items::food::Food;
use crate::items::item_spec::ItemSpec;
use crate::items::reagents::Reagent;
use crate::items::weapons::{Weapon, DAMAGE_DIE};
use crate::items::Item;
use crate::pc::pc_stat::PCStat;
use crate::utils::{some_if, split_operator};

use super::funds::short_funds;

impl IntoView for &Item {
    fn into_view(self) -> View {
        let colour = self.quality.colour();
        let item_type = match &self.spec {
            ItemSpec::Weapon(x) => match x.as_stat() {
                PCStat::DEX => "dex weapon",
                _ => "str weapon",
            },
            ItemSpec::Buff(_) => "potion",
            _ => self.spec.as_ref(),
        };
        let subtext = format!("{}, {item_type}", self.quality);
        let price = self.price;
        let specific_view = match &self.spec {
            ItemSpec::Weapon(weap) => weapon_view(weap).into_view(),
            ItemSpec::Buff(buff) => potion_view(buff).into_view(),
            ItemSpec::Food(food) => food_view(food).into_view(),
            ItemSpec::Consumable(effect) => effect.to_string().into_view(),
            ItemSpec::Reagent(reagent) => reagent_view(reagent).into_view(),
            _ => ().into_view(),
        };

        view! {
            <div class= "flex flex-col h-full justify-center px-2 text-start">
                <div class=format!("truncate uppercase {colour} title")>
                    { self.name.clone() }
                </div>
                <div class= "italic text-sm">
                    { subtext }
                </div>
                    { specific_view }
                <div class= "">
                    { short_funds(move || price) }
                </div>
            </div>
        }
        .into_view()
    }
}

fn weapon_view(spec: &Weapon) -> impl IntoView {
    let damage = format!("{} + {}", DAMAGE_DIE[spec.as_damage()], spec.as_stat());
    view! {
        <div> { damage } </div>
    }
}

fn food_view(Food { buff, fatigue }: &Food) -> impl IntoView {
    let buff = buff.as_ref().map(|buff| buff.to_string()).into_view();
    let fatigue = format!("Removes {fatigue} fatigue.");
    view! {
        { buff }
        { fatigue }
    }
}

fn potion_view(buff: &Buff) -> impl IntoView {
    let effect = buff
        .effect
        .as_ref()
        .map(|x| {
            view! {
            <div> { format!("Effect: {x}") } </div>
            }
        })
        .into_view();
    let stats = buff
        .stats
        .map(|x| {
            view! {
            <div> { x.to_string() } </div>
            }
        })
        .into_view();
    view! {
        <div>
            { stats }
            { effect }
        </div>
    }
}

fn reagent_view(reagent: &Reagent) -> impl IntoView {
    reagent
        .iter()
        .filter_map(|(sub, x)| {
            some_if(x > 0).map(|_| {
                let (op, num) = split_operator(x as i32);
                format!("{sub} {op}{num}")
            })
        })
        .reduce(|mut acc, e| {
            acc.push_str(", ");
            acc.push_str(&e);
            acc
        })
}
