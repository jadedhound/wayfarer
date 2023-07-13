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
    "rounded border-btnborder border-2 aspect-square flex items-center justify-center ";

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
                            <div class= BOX_CSS.plus("bg-btn")>
                                <div> {name} </div>
                            </div>
                        </A>
                    }
                })
                .collect_view(cx)
        })
    };

    view! { cx,
        <div class= "px-2 py-4 flex border-b-2 border-btnborder">
            <h3 class= "grow"> "WAYFARER" </h3>
            <A href= "/settings" class= "flex items-center">
                <svg viewBox="0 0 24 24" stroke-width="1.5" class="w-8 h-8 stroke-btnborder">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z" />
                  <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
            </A>
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
                <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
            </button>
            <div class= "mt-2">
                <div class= "flex flex-col items-center justify-center p-4 gap-4">
                    <h4> "Create Character" </h4>
                    <label class="flex gap-2">
                        <input
                            type="text"
                            class="text-slate-900 text-center w-full"
                            spellcheck="false"
                            prop:value=move || name.get()
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                        />
                        <button
                            class= "bg-slate-900 rounded-full p-2 flex flex-centered"
                            on:click=move |_| set_name.set(rand_name())
                        >
                            <div class= "w-8 h-8" inner_html=svg::DIE />
                        </button>
                    </label>
                    <button
                        class= "w-full py-2 bg-slate-900"
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
                        <div on:click=move |_| { set_modal.set(false) } class=BOX_CSS.plus("bg-btn")>
                            <svg viewBox="0 0 24 24" stroke-width="1.5" class="w-12 h-12 stroke-btnborder">
                              <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                            </svg>
                        </div>
                    }.into_view(cx)
                }}
            }.into_view(cx)
        }}
        <CreatePCModal hidden=hidden_modal set_hidden=set_modal />
    }
}
