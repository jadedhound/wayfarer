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
use crate::pc::pc_class::PCClassRef;
use crate::rand::Rand;
use crate::utils::index_map::IndexMap;
use crate::utils::{db, expect_rw, some_if};
use crate::views::delete_btn::{delete_btn, delete_btn_show};
use crate::views::modal::{modal_grey_screen, ModalState};
use crate::views::revealer::revealer_screen;

mod create_pc;
pub mod pc_basic;

const LOCKOUT_MINS: f64 = 0.0 * 60000.0;

#[rustfmt::skip]
const NAMES: [&str; 23] = [
    "Abigail","Emilia","Allison","Clara","Leah",
    "Myla","Ryanna","Valerie","Bram","Abram","Astin",
    "Bradyn","Cartus","Eric","Gavin","Han","Jax",
    "Jovan","Liam","Remus","Sebastion","Xander","Havy"
];

/// Keeps a timestamp of when the creating a new pc is acceptable again
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NewPCTimeout(pub f64);

/// Array with overview of PCs.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCList(pub IndexMap<PCBasic>);

/// PC overview and management.
pub fn lobby() -> impl IntoView {
    let pc_list = expect_rw::<PCList>();
    let pc_btns = move || pc_list.with(|list| list.0.iter().map(pc_btn).collect_view());

    view! {
        <div class= "p-2 flex border-b-2 border-amber-600">
            <h3 class= "grow font-regal"> "Wayfarer" </h3>
        </div>
        <div class= "grid grid-cols-2 gap-6 p-2">
            { pc_btns }
            { create_pc_btn() }
        </div>
        <div class= "psuedo h-px grow" />
        { build_info }
        { create_pc_modal }
        { modal_grey_screen }
        { revealer_screen }
    }
}

/// Displays a PC created by the user.
fn pc_btn((id, pc_basic): (usize, &PCBasic)) -> impl IntoView {
    let delete_pc = move || {
        // Remove from pc list.
        expect_rw::<PCList>().update(|pc_list| {
            pc_list.0.remove(id);
        });
        spawn_local(async move {
            let rm_db_ids = [id.to_string(), format!("{id}_journals")];
            if let Err(e) = db::remove(rm_db_ids.iter()).await {
                error!("{e}")
            }
        });
    };
    let name = pc_basic.name.clone();
    let (class_icon, colour) = match pc_basic.class {
        PCClassRef::Fighter => (icons::WARRIOR, "bg-amber-900"),
        PCClassRef::Rogue => (icons::ROGUE, "bg-red-800"),
        PCClassRef::Mage => (icons::MAGE, "bg-sky-800"),
        PCClassRef::Cleric => (icons::CLERIC, "bg-yellow-700"),
    };

    view! {
        <div class= "relative">
            <A href=format!("/pc/{id}") on:contextmenu=delete_btn_show('p', id)>
                <div class=format!("relative aspect-square btn {colour} flex-center overflow-hidden")>
                    <div class= "w-24 top-0 fill-black opacity-40" inner_html=class_icon />
                    <h4 class= "absolute text-center line-clamp-3"> { name } </h4>
                </div>
            </A>
            { delete_btn('p', id, delete_pc) }
        </div>
    }
}

/// Returns the wait time before another PC can be created.
fn cannot_create_pc() -> Option<u8> {
    let pc_timeout = expect_rw::<NewPCTimeout>();
    let time = pc_timeout.with(|time| time.0);
    let diff = time - js_sys::Date::now();
    let mins = (diff / 60000.0) as u8;
    // Wait 30 seconds and then refresh the timeout (thus the view).
    some_if(mins > 0).map(|_| {
        spawn_local(async move {
            sleep(Duration::from_secs(30)).await;
            pc_timeout.update(|time| {
                time.0 += 1.0;
            });
        });
        mins
    })
}

fn create_pc_btn() -> impl IntoView {
    let pc_basic = create_rw_signal(PCBasic::default());
    provide_context(pc_basic);
    let cannot_create = cannot_create_pc();
    let inner_text = match cannot_create {
        Some(timeout) => format!("{timeout} mins").into_view(),
        None => view! {
            <div class= "w-12" inner_html=icons::PLUS />
        }
        .into_view(),
    };
    let open_modal = move || {
        let name = Rand::with(|rand| rand.pick(&NAMES).to_string());
        pc_basic.update(|x| x.name = name);
        ModalState::open(0)
    };

    view! {
        <button
            class= "btn bg-surface flex-center flex-col aspect-square"
            on:click=move |_| open_modal()
            disabled=move || cannot_create.is_some()
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
