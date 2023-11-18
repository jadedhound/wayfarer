use leptos::*;

use crate::icons;
use crate::lobby::pc_basic::{PCBasic, NAMES};
use crate::lobby::{NewPCTimeout, PCList, LOCKOUT_MINS};
use crate::pc::class::PCClassRef;
use crate::rand::Rand;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;
use crate::views::checkbox::Checkbox;
use crate::views::modal::{ModalCenter, ModalLocation, ModalState};

pub(super) fn create_pc_modal() -> impl IntoView {
    view! {
        <ModalCenter location=ModalLocation::CreatePC>
            <h4 class= "text-center"> "Create Character" </h4>
            { name_input }
            { class_radio(PCClassRef::Fighter) }
            { class_radio(PCClassRef::Rogue) }
            { class_radio(PCClassRef::Mage) }
            { class_radio(PCClassRef::Cleric) }
            { create_btn }
        </ModalCenter>
    }
}

fn name_input() -> impl IntoView {
    let pc_basic = PCBasic::expect();
    let randomise_name = move || {
        let name = Rand::with(|rand| rand.pick(&NAMES).to_string());
        pc_basic.update(|x| x.name = name);
    };

    view! {
        <div class= "flex gap-2 w-full">
            <input
                type="text"
                class="input text-center grow w-12"
                spellcheck="false"
                prop:value=move || pc_basic.with(|x| x.name.clone())
                on:input=move |ev| pc_basic.update(|x| x.name = event_target_value(&ev))
                required
                maxlength=30
            />
            <button
                class= ""
                on:click=move |_| randomise_name()
            >
                <div class= "w-8 fill-sky-500" inner_html=icons::DIE />
            </button>
        </div>
    }
}

fn class_radio(class: PCClassRef) -> impl IntoView {
    let pc_basic = PCBasic::expect();
    let name = class.as_ref().to_owned();
    let chose_class = move || pc_basic.update(|x| x.class = class);
    let is_selected = PCBasic::slice(move |basic| basic.class == class);
    let desc = match class {
        PCClassRef::Fighter => {
            "You're quick, strong and militant. You excel in the use of deadly weaponry."
        }
        PCClassRef::Rogue => {
            "You’re sly, cunning and precise. You excel in subterfuge and stealth."
        }
        PCClassRef::Mage => {
            "You’re clever, eccentric and studious. You excel in the use of the arcane."
        }
        PCClassRef::Cleric => {
            "You’re devoted, stalwart and divine. Your fervour grants you providential powers."
        }
    };

    view! {
        <Checkbox
            checked=is_selected
            checked_colour= "border-yellow-500 text-yellow-500"
            on_click=chose_class
        >
            <div class= "uppercase">
                { name }
            </div>
            <div class= "font-sans" hidden=move || !is_selected.get()>
                { desc }
            </div>
        </Checkbox>
    }
}

fn create_btn() -> impl IntoView {
    let (pc_basic, pc_list, new_pc_timeout) =
        (PCBasic::expect(), PCList::expect(), NewPCTimeout::expect());
    let disabled = PCBasic::slice(|basic| basic.name.is_empty());
    // Custom revealer is needed because z-indexes stack ontop of one another.
    // a: z-1; b: z-2; c: z-3 (but child of a) would mean b is ontop, then c and lastly a.
    let rev_shown = RwSignal::new(false);
    let create_pc = move |_| {
        rev_shown.set(false);
        pc_list.update_discard(|list| list.0.add(pc_basic.get()));
        new_pc_timeout.update(|time| {
            // 10 secs of padding is needed due to rounding after division
            time.0 = LOCKOUT_MINS + js_sys::Date::now() + 10000.0;
        });
        ModalState::hide();
    };

    view! {
        <div class= "flex gap-2 items-center">
            <div class= "w-6 fill-red-500 stroke-transparent" inner_html=icons::CAUTION />
            <div class= "w-12 grow text-red-300">
                "You can only create a character once every 15 minutes."
            </div>
        </div>
        <div class= "relative">
            <button
                class= "btn-surface bg-zinc-700 w-full"
                disabled=disabled
                on:click=move |_| rev_shown.set(true)
            >
                "CREATE"
            </button>
            <button
                class= "btn-surface bg-green-800 absolute top-0 left-0 w-full h-full z-[31]"
                hidden=move || !rev_shown.get()
                on:click=create_pc
            >
                "CONFIRM"
            </button>
            <button
                class= "fixed z-30 top-0 left-0 h-full w-full psuedo"
                hidden=move || !rev_shown.get()
                on:click=move |_| rev_shown.set(false)
            />
        </div>
    }
}
