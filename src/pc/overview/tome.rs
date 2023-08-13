use leptos::*;

use crate::items::tome::{Tome, DC_BY_QUALITY};
use crate::items::Item;
use crate::rand::rand_context;
use crate::svg;
use crate::tables::spell_failure::spell_failure;
use crate::utils::rw_context;
use crate::views::modal::{ModalCentered, ModalState};

#[derive(Default)]
struct SpellFailure(String, String);

#[component]
pub(super) fn TomeView<'a>(cx: Scope, item: &'a Item, tome: &'a Tome) -> impl IntoView {
    let name = item.name.to_uppercase();
    let quality = item.quality;
    let Tome { stat, effect } = tome.clone();
    let dc = format!("{} DC {}", stat, DC_BY_QUALITY[quality as usize]);
    let spellfail = create_rw_signal(cx, SpellFailure::default());
    provide_context(cx, spellfail);

    view! { cx,
        <div class= "flex">
            <div class= "flex flex-col rounded-l border-y-2 border-l-2 border-zinc-700 px-2 w-full">
                <div> { name } </div>
                <div> { effect } </div>
                <div class= "font-sans text-center mt-2"> { dc } </div>
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
                class= "rounded-r bg-red-800 flex-centered w-12"
            >
                <div class= "svg w-6" inner_html=svg::LIGHTNING_BOLT />
            </button>
        </div>
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
