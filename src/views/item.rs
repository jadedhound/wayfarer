use leptos::*;

use crate::items::armour::{Armour, ArmourClass, BodyPart};
use crate::items::buffs::{Buff, FeatOrStat};
use crate::items::item_specs::ItemSpec;
use crate::items::weapons::{Weapon, DAMAGE_DIE};
use crate::items::Item;
use crate::pc::PCStat;
use crate::utils::capitalise;
use crate::views::Funds;

impl IntoView for &Item {
    fn into_view(self, cx: Scope) -> View {
        let name = capitalise(&self.name);
        let colour = self.quality.colour();
        let item_type = match &self.spec {
            ItemSpec::Head(_) => "fancy headwear",
            ItemSpec::Weapon(x) => match x.as_stat() {
                PCStat::DEX => "dex weapon",
                _ => "str weapon",
            },
            ItemSpec::Armour(x) => match x.body_part {
                BodyPart::Held => "held armour",
                BodyPart::Body => match x.class {
                    ArmourClass::Light => "light body armour",
                    ArmourClass::Medium => "medium body armour",
                    ArmourClass::Heavy => "heavy body armour",
                },
                BodyPart::Legs => match x.class {
                    ArmourClass::Light => "light leg armour",
                    ArmourClass::Medium => "medium leg armour",
                    ArmourClass::Heavy => "heavy leg armour",
                },
            },
            ItemSpec::Buff(_) => "potion",
            ItemSpec::Stackable(_, _) => "simple",
            _ => self.spec.as_ref(),
        };
        let subtext = format!("{}, {item_type}", self.quality);
        let price = self.price;
        let specific_view = match &self.spec {
            ItemSpec::Weapon(spec) => view! { cx, <WeaponView spec /> }.into_view(cx),
            ItemSpec::Armour(spec) => view! { cx, <ArmourView spec /> }.into_view(cx),
            ItemSpec::Buff(condition) => view! { cx, <PotionView condition /> }.into_view(cx),
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

#[component]
fn ArmourView<'a>(cx: Scope, spec: &'a Armour) -> impl IntoView {
    let stats = spec
        .stats
        .string_iter()
        .map(|x| {
            view! { cx,
                <div> { x } </div>
            }
        })
        .collect_view(cx);

    view! { cx,
        <div class= "grid grid-cols-2">
            { stats }
        </div>
    }
}

#[component]
fn PotionView<'a>(cx: Scope, condition: &'a Buff) -> impl IntoView {
    let effect = match &condition.effect {
        FeatOrStat::Feat(x) => format!("the following {} feat, which allows {}", x.name, x.effect),
        FeatOrStat::Stat(x) => x.string_iter().fold(String::new(), |mut acc, e| {
            acc.push_str(&e);
            acc
        }),
    };
    view! { cx,
        <div>
            { format!("Consume: {effect}.") }
        </div>
    }
}
