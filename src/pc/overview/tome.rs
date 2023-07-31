use leptos::*;

use crate::items::item_specs::Tome;
use crate::items::tome::DC_BY_QUALITY;
use crate::items::Item;
use crate::pc::session::PCSession;
use crate::pc::PCStat;
use crate::rand::rand_context;
use crate::tables::spell_failure::spell_failure;
use crate::utils::{capitalise, rw_context};
use crate::views::modal::{ModalCentered, ModalState};

#[derive(Default)]
struct SpellFailure(String, String);

#[component]
pub(super) fn TomeView<'a>(cx: Scope, item: &'a Item, tome: &'a Tome) -> impl IntoView {
    let sesh = rw_context::<PCSession>(cx);
    let name = capitalise(&item.name);
    let quality = item.quality;
    let Tome { stat, effect } = tome.clone();
    let dc = move || {
        let dc = sesh.with(|sesh| {
            let base = DC_BY_QUALITY[quality as usize];
            let sorc = sesh.stats[PCStat::Sorcery.index()] as u8;
            base - sorc
        });
        format!("{} DC {}", stat, dc)
    };
    let spellfail = create_rw_signal(cx, SpellFailure::default());
    provide_context(cx, spellfail);

    view! { cx,
        <div class= "flex flex-col rounded-t border-x-2 border-t-2 border-zinc-700 pb-2 px-2">
            <div> { name } </div>
            <div> { effect } </div>
            <div class= "font-sans text-center"> { dc } </div>
        </div>
        <button
            on:click=move |_| {
                let (who, what) = rand_context(cx, spell_failure);
                spellfail.update(|x| {
                    x.0 = who.to_string();
                    x.1 = what.to_string();
                });
                ModalState::open(cx, 0)
            }
            class= "bg-red-800 font-sans rounded-b w-full h-10"
        >
            "SPELL FAILURE"
        </button>
        <Modal />
    }
}

#[component]
fn Modal(cx: Scope) -> impl IntoView {
    let details = move || {
        rw_context::<SpellFailure>(cx).with(|x| {
            view! { cx,
                <div> { &x.0 } </div>
                <div> { &x.1 } </div>
            }
        })
    };
    view! { cx,
        <ModalCentered title=|| "SPELL FAILURE" id=0>
            { details }
        </ModalCentered>
    }
}
