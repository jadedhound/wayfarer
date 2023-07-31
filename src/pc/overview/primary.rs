use std::cmp::min;

use leptos::*;

use crate::items::item_specs::ItemSpec;
use crate::items::weapons::{Weapon, DAMAGE_DIE};
use crate::pc::equip_slot::EquipSlot;
use crate::pc::overview::tome::TomeView;
use crate::pc::session::PCSession;
use crate::pc::{PCStat, PC};
use crate::utils::{add_operator, capitalise, rw_context};

#[component]
pub(super) fn Primary(cx: Scope) -> impl IntoView {
    move || {
        rw_context::<PC>(cx).with(|pc| {
            if let Some(item) = pc.get_equipment(EquipSlot::MainHand) {
                match &item.spec {
                    ItemSpec::Weapon(weap) => view! { cx,
                        <WeaponAtk name=item.name.clone() weap=*weap />
                    },
                    ItemSpec::Tome(tome) => view! { cx,
                        <TomeView item tome />
                    },
                    _ => view! { cx, <Bash /> }.into_view(cx),
                }
            } else {
                view! { cx, <NameAndDmg name="Unarmed Strike".into() dmg="1".into() /> }
                    .into_view(cx)
            }
        })
    }
}

#[component]
fn NameAndDmg(cx: Scope, name: String, dmg: String) -> impl IntoView {
    view! { cx,
        <div class= " rounded border-2 border-zinc-700 flex items-center font-sans">
            <div class= "ml-2 py-2 w-12 grow"> { capitalise(&name) } </div>
            <div class= "border-l-2 border-zinc-700 px-4 w-28 text-center">
                { dmg }
            </div>
        </div>
    }
}

#[component]
fn Bash(cx: Scope) -> impl IntoView {
    let strength = rw_context::<PCSession>(cx).with(|sesh| sesh.stats[PCStat::STR.index()]);
    let dmg = min(strength, 1);
    view! { cx,
        <NameAndDmg name="Improvised Bash".into() dmg=dmg.to_string() />
    }
}

#[component]
fn WeaponAtk(cx: Scope, name: String, weap: Weapon) -> impl IntoView {
    let sesh = rw_context::<PCSession>(cx);
    let pc = rw_context::<PC>(cx);
    let dmg_incr = move || add_operator(sesh.with(|sesh| sesh.stats[weap.as_stat().index()]));
    move || {
        pc.with(|pc| {
            let off_weap = pc.get_equipment(EquipSlot::OffHand).and_then(|x| {
                let dmg = x.spec.as_weapon()?.as_damage();
                Some((x, dmg))
            });
            if let Some((off, off_dmg)) = off_weap {
                let name = format!("{} and {} assault", name, off.name);
                let dmg = format!(
                    "Best of ({}/{}){}",
                    DAMAGE_DIE[weap.as_damage()],
                    DAMAGE_DIE[off_dmg],
                    dmg_incr()
                );
                view! { cx,
                    <NameAndDmg name dmg />
                }
            } else {
                let name = format!("{} attack", name);
                let dmg = format!("{}{}", DAMAGE_DIE[weap.as_damage()], dmg_incr());
                view! { cx,
                    <NameAndDmg name dmg />
                }
            }
        })
    }
}
