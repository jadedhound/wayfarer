use leptos::*;

use crate::icons;
use crate::lobby::pc_basic::{PCBasic, NAMES};
use crate::lobby::{NewPCTimeout, PCList, LOCKOUT_MINS};
use crate::pc::class::PCClassRef;
use crate::rand::Rand;
use crate::utils::expect_rw;
use crate::utils::rw_utils::RwUtils;
use crate::views::modal::{ModalCenter, ModalState};

const MAX_NAME_LEN: usize = 30;

pub(super) fn create_pc_modal() -> impl IntoView {
    view! {
        <ModalCenter id=10>
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
                maxlength=MAX_NAME_LEN
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
    let pc_basic = expect_rw::<PCBasic>();
    let name = class.as_ref().to_owned();
    let click = move || pc_basic.update(|x| x.class = class);
    let is_selected = create_memo(move |_| class == pc_basic.with(|pc| pc.class));
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
    let select_style = move || {
        if is_selected.get() {
            "border-yellow-400 text-yellow-400"
        } else {
            "border-zinc-500 text-zinc-500"
        }
    };

    view! {
        <button
            class=move || format!("border-2 rounded p-2 {}", select_style())
            on:click=move |_| click()
        >
            <div class= "font-tight uppercase">
                { name }
            </div>
            <div hidden=move || !is_selected.get()>
                { desc }
            </div>
        </button>
    }
}

fn create_btn() -> impl IntoView {
    let pc_basic = expect_rw::<PCBasic>();
    let cannot_create = create_memo(move |_| pc_basic.with(|pc| pc.name.is_empty()));
    let create_pc = move || {
        expect_rw::<PCList>().update(|list| list.0.add(pc_basic.get()));
        expect_rw::<NewPCTimeout>().update(|time| {
            // 10 secs of padding is needed due to rounding after division
            time.0 = LOCKOUT_MINS + js_sys::Date::now() + 10000.0;
        });
        ModalState::hide();
    };

    view! {
        <button
            class= "py-2 btn-surface bg-green-800"
            on:click=move |_| create_pc()
            disabled=cannot_create
        >
            "CREATE"
        </button>
    }
}
