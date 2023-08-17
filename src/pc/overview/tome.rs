use leptos::*;

use crate::items::tome::{Tome, DC_BY_QUALITY};
use crate::items::Item;
use crate::svg;
use crate::tables::spell_failure::spell_failure;
use crate::utils::expect_rw;
use crate::views::modal::{ModalCentered, ModalState};

#[derive(Default)]
struct FailureResult(String, String);

pub(super) fn tome_view<'a>(item: &'a Item, tome: &'a Tome) -> impl IntoView {
    let effect = format!("{}.", tome.effect);
    let quality = item.quality;
    let dc = DC_BY_QUALITY[quality as usize];
    let spellfail = create_rw_signal(FailureResult::default());
    provide_context(spellfail);
    let on_failure = move || {
        let (who, what) = spell_failure();
        spellfail.update(|x| {
            x.0 = who.to_string();
            x.1 = what.to_string();
        });
        ModalState::open(10)
    };

    view! {
        <div class= "flex">
            <div class= "flex flex-col p-2 w-12 grow">
                <div class= "uppercase title"> { item.name.clone() } </div>
                <div> { effect } </div>
            </div>
            <button
                on:click=move |_| on_failure()
                class= "flex-centered flex-col w-16 gap-2"
            >
                <div class= "font-sans">
                    <div> "INT DC" </div>
                    <div class= "-mt-2"> { dc } </div>
                </div>
                <div class= "fill-red-500 w-6" inner_html=svg::LIGHTNING_BOLT />
            </button>
            { modal }
        </div>
    }
}

fn modal() -> impl IntoView {
    let details = move || {
        expect_rw::<FailureResult>().with(|x| {
            view! {
                <div> { &x.0 } </div>
                <div> { &x.1 } </div>
            }
        })
    };
    view! {
        <ModalCentered title=|| "SPELL FAILURE" id=10>
            { details }
        </ModalCentered>
    }
}
