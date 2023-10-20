use std::time::Duration;

use const_format::formatcp;
use gloo::timers::future::sleep;
use leptos::logging::error;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use self::pc_basic::PCBasic;
use crate::icons;
use crate::lobby::create_pc::create_pc_modal;
use crate::lobby::pc_basic::NAMES;
use crate::pc::class::PCClassRef;
use crate::rand::Rand;
use crate::utils::db::DBKey;
use crate::utils::index_map::IndexMap;
use crate::utils::rw_utils::RwUtils;
use crate::utils::RwSignalEnhance;
use crate::views::delete_confirm::DeleteModal;
use crate::views::modal::ModalState;

mod create_pc;
pub mod pc_basic;

/// 15 min lock out to prevent PC spamming.
const LOCKOUT_MINS: f64 = {
    if cfg!(debug_assertions) {
        0.0
    } else {
        15.0 * 60000.0
    }
};

/// Keeps a timestamp of when the creating a new pc is acceptable again
#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct NewPCTimeout(pub f64);

impl RwUtils for NewPCTimeout {
    type Item = Self;
}

/// Array with overview of PCs.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCList(pub IndexMap<PCBasic>);

impl RwUtils for PCList {
    type Item = Self;
}

/// PC overview and management.
pub fn lobby() -> impl IntoView {
    PCBasic::provide();
    let pc_list = PCList::expect();
    let pc_btns = move || pc_list.with(|list| list.0.iter().map(pc_btn).collect_view());
    // Set what happens when a PC is confirmed for deletion.
    DeleteModal::set_effect(move |id| {
        // Remove from pc list.
        pc_list.update_discard(|pc_list| pc_list.0.remove(id));
        spawn_local(async move {
            for ele in [DBKey::PC(id), DBKey::PCJournal(id)] {
                if let Err(e) = ele.remove().await {
                    error!("DB Remove Error: {e}")
                }
            }
        });
    });

    view! {
        <div class= "fixed top-0 left-0 h-16 w-full bg-black px-2 flex-center border-b border-amber-600">
            <h3> "Wayfarer" </h3>
        </div>
        <div class= "psuedo h-16" />
        { pc_btns }
        { create_pc_btn }
        <div class= "psuedo h-px grow" />
        { build_info }
        { create_pc_modal }
    }
}

/// Displays a PC created by the user.
fn pc_btn((id, pc_basic): (usize, &PCBasic)) -> impl IntoView {
    let show_delete_modal = move |_| DeleteModal::show(id);
    let name = pc_basic.name.clone();
    let (class_icon, colour) = match pc_basic.class {
        PCClassRef::Fighter => (icons::WARRIOR, "fill-amber-700"),
        PCClassRef::Rogue => (icons::ROGUE, "fill-red-600"),
        PCClassRef::Mage => (icons::MAGE, "fill-sky-500"),
        PCClassRef::Cleric => (icons::CLERIC, "fill-yellow-500"),
    };

    view! {
        <div class= "flex gap-3 p-2 btn bg-surface">
            <div class=format!("w-8 {colour}") inner_html=class_icon />
            <A href=format!("/pc/{id}") class= "w-12 grow truncate">
                <h5> { name } </h5>
            </A>
            <button on:click=show_delete_modal>
                <div class= "w-5 fill-red-600" inner_html=icons::TRASH />
            </button>
        </div>
    }
}

fn create_pc_btn() -> impl IntoView {
    let timeout = NewPCTimeout::expect();
    let pc_basic = PCBasic::expect();
    let mins_left = NewPCTimeout::slice(|timeout| {
        let diff = timeout.0 - js_sys::Date::now();
        (diff / 60000.0) as u8
    });
    let inner_text = move || {
        if mins_left.get() > 0 {
            format!("{} MIN COOLDOWN", mins_left.get()).into_view()
        } else {
            view! {
                <div class= "w-6" inner_html=icons::PLUS />
                <div class= "">
                    "CREATE"
                </div>
                <div class= "w-6 psuedo" />
            }
            .into_view()
        }
    };
    let open_modal = move |_| {
        let name = Rand::with(|rand| rand.pick(&NAMES).to_string());
        pc_basic.update(|x| x.name = name);
        ModalState::show(10)
    };
    create_effect(move |_| {
        if mins_left.get() > 0 {
            spawn_local(async move {
                sleep(Duration::from_secs(30)).await;
                timeout.update_discard(|time| time.0 += 1.0);
            });
        }
    });

    view! {
        <button
            class= "btn bg-green-800 flex-center gap-1 py-2"
            on:click=open_modal
            disabled=move || { mins_left.get() > 0 }
        >
            { inner_text }
        </button>
    }
}

fn build_info() -> impl IntoView {
    const VERSION: &str = formatcp!("v{}", env!("CARGO_PKG_VERSION"));

    view! {
        <a href= "https://codeberg.org/jadehound/wayfarer" target= "_blank">
            <div class= "flex-center font-tight gap-2 py-2">
                <div> { VERSION } </div>
                <div class= "w-4" inner_html=icons::CODEBERG />
                <div> "SOURCE" </div>
            </div>
        </a>
    }
}
