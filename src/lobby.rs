use std::time::Duration;

use const_format::concatcp;
use gloo::timers::future::sleep;
use leptos::*;
use leptos_router::*;

use crate::assets::NAMES;
use crate::rand::rand_context;
use crate::state::{NewPCTimeout, PCList};
use crate::svg;
use crate::utils::{provide_saved, rw_context};
use crate::views::modal::{ModalCentered, ModalState};

const LOCKOUT_MINS: f64 = 0.0 * 60000.0;
const BOX_CSS: &str =
    "rounded border-zinc-200 border-2 aspect-square flex items-center justify-center";

#[component]
pub fn Lobby(cx: Scope) -> impl IntoView {
    let pc_list = move || {
        rw_context::<PCList>(cx).with(|list| {
            list.get_all()
                .map(|pc| {
                    let link = format!("/pc/{}", pc.id);
                    let name = pc.name.clone();
                    view! { cx,
                        <A href=link>
                            <div class= concatcp!(BOX_CSS, " bg-red-900")>
                                <div> {name} </div>
                            </div>
                        </A>
                    }
                })
                .collect_view(cx)
        })
    };

    view! { cx,
        <div class= "px-2 py-4 flex border-b-2 border-red-900">
            <h3 class= "grow"> "WAYFARER" </h3>
        </div>
        <div class= "h-full grid grid-cols-2 gap-6 p-6">
            { pc_list }
            <CreatePCButton />
        </div>
    }
}

/// Returns the wait time before another PC can be created.
fn cannot_create_pc(cx: Scope) -> Option<u8> {
    let pc_timeout = rw_context::<NewPCTimeout>(cx);
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

const CREATE_PC_MODAL: u8 = 0;

#[component]
fn CreatePCButton(cx: Scope) -> impl IntoView {
    let loading = create_resource(
        cx,
        || (),
        move |_| async move { provide_saved(cx, "new_pc_timeout", || NewPCTimeout(0.0)).await },
    );

    view! { cx,
        {move || match loading.read(cx) {
            None => view!{cx,}.into_view(cx),
            Some(_) => view!{cx,
                {move || match cannot_create_pc(cx) {
                    Some(timeout) => view!{
                        cx,
                        <div class=concatcp!(BOX_CSS, " bg-zinc-900")>
                            "Please wait " {timeout} " mins"
                        </div>
                    }.into_view(cx),
                    None => view!{
                        cx,
                        <button
                            on:click=move |_| { ModalState::open(cx, CREATE_PC_MODAL) }
                            class=concatcp!(BOX_CSS, " bg-red-900")
                        >
                            <div class= "w-12 svg" inner_html=svg::PLUS />
                        </button>
                    }.into_view(cx)
                }}
            }.into_view(cx)
        }}
        <CreatePCModal/>
    }
}

#[component]
fn CreatePCModal(cx: Scope) -> impl IntoView {
    let create_pc = move |name: String| {
        rw_context::<PCList>(cx).update(|list| {
            list.add(name);
        });
        rw_context::<NewPCTimeout>(cx).update(|time| {
            // 10 secs of padding is needed due to rounding after division
            time.0 = LOCKOUT_MINS + js_sys::Date::now() + 10000.0;
        });
        ModalState::dismiss(cx);
    };
    let rand_name = move || rand_context(cx, |rng| rng.pick(&NAMES).to_string());
    let (name, set_name) = create_signal(cx, rand_name());

    view! {
        cx,
        <ModalCentered title=|| "Create PC" id=CREATE_PC_MODAL>
            <div class= "flex gap-2 w-full">
                <input
                    type="text"
                    class="text-slate-900 text-center grow w-12 rounded"
                    spellcheck="false"
                    prop:value=move || name.get()
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                />
                <button
                    class= "bg-slate-900 rounded-full h-12 w-12 flex flex-centered"
                    on:click=move |_| set_name.set(rand_name())
                >
                    <div class= "w-8 svg" inner_html=svg::DIE />
                </button>
            </div>
            <button
                class= "w-full rounded py-2 bg-slate-900"
                on:click=move |_| create_pc(name.get())
            >
                "Create"
            </button>
        </ModalCentered>
    }
}
