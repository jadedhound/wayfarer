use leptos::*;

use crate::items::buffs::Buff;
use crate::items::food::Food;
use crate::items::item_spec::ItemSpec;
use crate::items::weapons::{Weapon, DAMAGE_DIE};
use crate::items::Item;
use crate::pc::PCStat;
use crate::views::Funds;

impl IntoView for &Item {
    fn into_view(self, cx: Scope) -> View {
        let name = self.name.to_uppercase();
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
            ItemSpec::Weapon(spec) => view! { cx, <WeaponView spec /> }.into_view(cx),
            ItemSpec::Buff(buff) => potion_view(cx, buff).into_view(cx),
            ItemSpec::Food(food) => food_view(cx, food).into_view(cx),
            ItemSpec::Consumable(effect) => effect.to_string().into_view(cx),
            _ => ().into_view(cx),
        };

        view! { cx,
            <div class= "flex flex-col h-full justify-center px-2 text-start">
                <div class=format!("truncate {colour}")>
                    { name }
                </div>
                <div class= "italic text-sm">
                    { subtext }
                </div>
                    { specific_view }
                <div class= "">
                    <Funds sup=move || price />
                </div>
            </div>
        }
        .into_view(cx)
    }
}

#[component]
fn WeaponView<'a>(cx: Scope, spec: &'a Weapon) -> impl IntoView {
    let damage = format!("{} + {}", DAMAGE_DIE[spec.as_damage()], spec.as_stat());
    view! { cx,
        <div> { damage } </div>
    }
}

fn food_view(cx: Scope, Food { buff, fatigue }: &Food) -> impl IntoView {
    let buff = buff.as_ref().map(|buff| buff.to_string()).into_view(cx);
    let fatigue = format!("Removes {fatigue} fatigue.");
    view! { cx,
        { buff }
        { fatigue }
    }
}

fn potion_view(cx: Scope, buff: &Buff) -> impl IntoView {
    let effect = buff
        .effect
        .as_ref()
        .map(|x| x.desc.clone().into_view(cx))
        .into_view(cx);
    let stats = buff.stats.map(|x| x.to_string()).into_view(cx);
    view! { cx,
        <div>
            { stats }
            { effect }
        </div>
    }
}
