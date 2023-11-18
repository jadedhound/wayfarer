use leptos::*;

use crate::icons;
use crate::pc::PC;
use crate::utils::rw_utils::RwUtils;
use crate::views::confirm_button::ConfirmButton;
use crate::views::revealer::RevLocation;
use crate::views::toast::Toast;

pub fn rally() -> impl IntoView {
    let pc = PC::expect();
    let rally = move || {
        pc.update(|pc| {
            pc.guard_dmg = 0;
            pc.fatigue += 1;
        });
        Toast::show("rally", "guard restored, faitgue added");
    };
    view! {
        <div class= "italic text-center">
            "Take a deep breath and gather yourself: restoring GUARD completely and adding FATIGUE into your inventory."
        </div>
        <ConfirmButton location=RevLocation::RallyConfirm on_click=rally>
            <div class= "flex-center gap-2">
                <div class= "w-5" inner_html=icons::FIST />
                "RALLY"
                <div class= "psuedo w-5" />
            </div>
        </ConfirmButton>
    }
}
