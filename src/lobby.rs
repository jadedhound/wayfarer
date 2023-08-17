use std::time::Duration;

use const_format::formatcp;
use gloo::timers::future::sleep;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::assets::NAMES;
use crate::rand::Rand;
use crate::svg;
use crate::utils::db::{self, provide_saved};
use crate::utils::expect_rw;
use crate::utils::index_map::IndexMap;
use crate::views::modal::{modal_grey_screen, ModalCentered, ModalState};
use crate::views::revealer::{Revealer, RevealerScreen};

const LOCKOUT_MINS: f64 = 0.0 * 60000.0;

/// Keeps a timestamp of when the creating a new pc is acceptable again
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NewPCTimeout(pub f64);

/// Array with overview of PCs.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PCList(pub IndexMap<String>);

/// Holds the name for the PC.
struct Name(String);

impl Name {
    fn reroll(&mut self) {
        self.0 = Rand::with(|rand| rand.pick(&NAMES)).to_string();
    }
}

/// PC overview and management.
pub fn lobby() -> impl IntoView {
    let pc_list = move || {
        expect_rw::<PCList>().with(|pc_list| {
            pc_list
                .0
                .iter()
                .map(|(id, name)| view! { <PCBtn id name=name.clone() /> })
                .collect_view()
        })
    };

    view! {
        <div class= "px-2 py-4 flex border-b-2 border-red-900">
            <h3 class= "grow"> "WAYFARER" </h3>
        </div>
        <div class= "grid grid-cols-2 gap-6 p-6">
            { pc_list }
            { create_pc_btn() }
        </div>
        <div class= "psuedo h-px grow" />
        { build_info() }
        { create_pc_modal() }
        { modal_grey_screen() }
        <RevealerScreen />
    }
}

/// Deletes all records of a PC.
fn delete_pc(id: usize) {
    // Remove from pc list.
    expect_rw::<PCList>().update(|pc_list| {
        pc_list.0.remove(id);
    });
    spawn_local(async move {
        let rm_db_ids = [id.to_string(), format!("{id}_journals")];
        if let Err(e) = db::remove(rm_db_ids.iter()).await {
            log::error!("{e}")
        }
    })
}

/// Displays a PC created by the user.
#[component]
fn PCBtn(id: usize, name: String) -> impl IntoView {
    let del_name = name.clone();

    view! {
        <div class= "relative">
            <A
                href=format!("/pc/{id}")
                on:contextmenu=move |_| Revealer::open('p', id)
            >
                <div class= "btn-zinc flex-centered aspect-square">
                    <div> { name } </div>
                </div>
            </A>
            <button
                class= "absolute inset-0 z-40"
                on:click=move |_| {
                    delete_pc(id);
                    Revealer::dismiss()
                }
                hidden=move || !Revealer::state('p', id)
            >
                <div class= "flex-centered bg-red-800 h-full rounded gap-x-2">
                    <div class= "w-6 svg" inner_html=svg::TRASH />
                    <div> { format!("Delete {del_name}?") } </div>
                </div>
            </button>
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
    if mins > 0 {
        spawn_local(async move {
            sleep(Duration::from_secs(30)).await;
            pc_timeout.update(|time| {
                time.0 += 1.0;
            });
        });
        Some(mins)
    } else {
        None
    }
}

fn create_pc_btn() -> impl IntoView {
    let name = create_rw_signal(Name(String::new()));
    provide_context(name);

    let loading = create_resource(
        || (),
        move |_| async move { provide_saved("new_pc_timeout", || NewPCTimeout(0.0)).await },
    );

    move || {
        loading.get().map(|_| {
            let cannot_create = cannot_create_pc();
            let inner_text = match cannot_create {
                Some(timeout) => view! {
                    <span> "Please wait" </span>
                    <span> { format!("{timeout} mins") } </span>
                }
                .into_view(),
                None => view! {
                    <div class= "w-12 svg" inner_html=svg::PLUS />
                }
                .into_view(),
            };
            view! {
                <button
                    class="btn-zinc flex-centered flex-col aspect-square"
                    on:click=move |_| {
                        name.update(|name| { name.reroll(); });
                        ModalState::open(0)
                    }
                    disabled=move || cannot_create.is_some()
                >
                    { inner_text }
                </button>
            }
        })
    }
}

fn create_pc_modal() -> impl IntoView {
    let name = expect_rw::<Name>();
    let create_pc = move |name: String| {
        expect_rw::<PCList>().update(|list| list.0.add(name));
        expect_rw::<NewPCTimeout>().update(|time| {
            // 10 secs of padding is needed due to rounding after division
            time.0 = LOCKOUT_MINS + js_sys::Date::now() + 10000.0;
        });
        ModalState::dismiss();
    };

    view! {
        <ModalCentered title=|| "Create PC" id=0>
            <div class= "flex gap-2 w-full">
                <input
                    type="text"
                    class="text-slate-900 text-center grow w-12 rounded"
                    spellcheck="false"
                    prop:value=move || name.with(|name| name.0.clone())
                    on:input=move |ev| name.update(|name| name.0 = event_target_value(&ev))
                />
                <button
                    class= "bg-slate-900 rounded-full h-12 w-12 flex flex-centered"
                    on:click=move |_| name.update(|name| name.reroll())
                >
                    <div class= "w-8 svg" inner_html=svg::DIE />
                </button>
            </div>
            <button
                class= "w-full rounded py-2 bg-slate-900"
                on:click=move |_| create_pc(name.with(|name| name.0.clone()))
            >
                "Create"
            </button>
        </ModalCentered>
    }
}

fn build_info() -> impl IntoView {
    const VERSION: &str = formatcp!("v{}", env!("CARGO_PKG_VERSION"));

    view! {
        <a href="https://codeberg.org/jadehound/wayfarer" target="_blank">
            <div class= "flex-centered font-sans gap-2 py-2">
                <div> { VERSION } </div>
                <div class= "w-4" inner_html=svg::CODEBERG />
                <div> "SOURCE" </div>
            </div>
        </a>
    }
}
