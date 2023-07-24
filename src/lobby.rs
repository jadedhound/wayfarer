use std::time::Duration;

use leptos::*;
use leptos_router::*;

use crate::assets::NAMES;
use crate::modal::*;
use crate::rand::rand_context;
use crate::state::{NewPCTimeout, PCList};
use crate::svg;
use crate::utils::{provide_saved, read_context, write_context, StrPlus};

const LOCKOUT_MINS: f64 = 0.0 * 60000.0;
const BOX_CSS: &str =
    "rounded border-zinc-200 border-2 aspect-square flex items-center justify-center ";

#[component]
pub fn Lobby(cx: Scope) -> impl IntoView {
    let pc_list = move || {
        read_context::<PCList>(cx).with(|list| {
            list.get_all()
                .map(|pc| {
                    let link = format!("/pc/{}", pc.id);
                    let name = pc.name.clone();
                    view! { cx,
                        <A href=link>
                            <div class= BOX_CSS.plus("bg-red-900")>
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
            {move || pc_list()}
            <CreatePCButton />
        </div>
    }
}

#[component]
fn CreatePCModal(
    cx: Scope,
    hidden: ReadSignal<bool>,
    set_hidden: WriteSignal<bool>,
) -> impl IntoView {
    let create_pc = move |name: String| {
        write_context::<PCList>(cx).update(|list| {
            list.add(name);
        });
        write_context::<NewPCTimeout>(cx).update(|time| {
            // 10 secs of padding is needed due to rounding after division
            time.0 = LOCKOUT_MINS + js_sys::Date::now() + 10000.0;
        });
        set_hidden.set(true);
    };
    let rand_name = move || rand_context(cx, |rng| rng.pick(&NAMES).to_string());
    let (name, set_name) = create_signal(cx, rand_name());

    view! {
        cx,
        <MiddleModal hidden=hidden>
            <button on:click=move |_| set_hidden.set(true) class= "absolute top-2 right-2">
                <div class= "svg w-8" inner_html=svg::CROSS />
            </button>
            <div class= "mt-2">
                <div class= "flex flex-col items-center justify-center p-4 gap-4">
                    <h4> "Create Character" </h4>
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
                </div>
            </div>
        </MiddleModal>
    }
}

#[component]
fn CreatePCButton(cx: Scope) -> impl IntoView {
    let loading = create_resource(
        cx,
        || (),
        move |_| async move { provide_saved(cx, "new_pc_timeout", NewPCTimeout(0.0)).await },
    );
    let (hidden_modal, set_modal) = create_signal(cx, true);
    let is_timed_out = move || {
        read_context::<NewPCTimeout>(cx).with(|time| {
            let diff = time.0 - js_sys::Date::now();
            let mins = (diff / 60000.0) as u8;
            if mins > 0 {
                spawn_local(async move {
                    gloo::timers::future::sleep(Duration::from_secs(30)).await;
                    write_context::<NewPCTimeout>(cx).update(|time| {
                        time.0 += 1.0;
                    });
                });
                Some(mins)
            } else {
                None
            }
        })
    };

    view! { cx,
        {move || match loading.read(cx) {
            None => view!{cx,}.into_view(cx),
            Some(_) => view!{cx,
                {move || match is_timed_out() {
                    Some(timeout) => view!{
                        cx,
                        <div class=BOX_CSS.plus("bg-zinc-900")>
                            "Please wait " {timeout} " mins"
                        </div>
                    }.into_view(cx),
                    None => view!{
                        cx,
                        <button on:click=move |_| { set_modal.set(false) } class=BOX_CSS.plus("bg-red-900")>
                            <div class= "w-12 svg" inner_html=svg::PLUS />
                        </button>
                    }.into_view(cx)
                }}
            }.into_view(cx)
        }}
        <CreatePCModal hidden=hidden_modal set_hidden=set_modal />
    }
}
