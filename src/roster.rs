use std::time::Duration;

use crate::{
    assets::NAMES,
    modal::*,
    utils::wyrand_context,
    wyrand::{self, WyRand},
};
use leptos::{ev::MouseEvent, *};
use leptos_router::*;

use crate::{
    state::{AppState, PCState, PChar},
    utils::{read_context, write_context},
};

const BOX_CSS: &str =
    "rounded border-btnborder border-2 aspect-square flex items-center justify-center ";

#[component]
pub fn Roster(cx: Scope) -> impl IntoView {
    let pc_list = move || {
        read_context::<PCState>(cx).with(|state| {
            state
                .0
                .iter()
                .map(|pc| {
                    view! { cx,
                        <div class= BOX_CSS.to_string() + "bg-btn">
                            <div> {pc.name.clone()} </div>
                        </div>
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
        write_context::<PCState>(cx).update(|state| {
            state.0.push(PChar::new(name));
        });
        write_context::<AppState>(cx).update(|state| {
            state.new_char_timeout = (15.2 * 60000.0) + js_sys::Date::now();
        });
        set_hidden.set(true);
    };
    let rand_name = move || wyrand_context(cx, |rng| rng.from_arr(&NAMES).to_string());
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
                        <span class=""> "Name" </span>
                        <input
                            type="text"
                            class="text-slate-900"
                            spellcheck="false"
                            prop:value=move || name.get()
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                        />
                        <div on:click=move |_| set_name.set(rand_name())>
                            <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                            </svg>
                        </div>
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
    let (hidden_modal, set_modal) = create_signal(cx, true);
    let is_timed_out = move || {
        read_context::<AppState>(cx).with(|state| {
            let dif = state.new_char_timeout - js_sys::Date::now();
            if dif > 0.0 {
                spawn_local(async move {
                    gloo::timers::future::sleep(Duration::from_secs(60)).await;
                    write_context::<AppState>(cx).update(|state| {
                        state.new_char_timeout += 1.0;
                    });
                });
                let mins = (dif / 60000.0) as u8;
                Some(mins)
            } else {
                None
            }
        })
    };

    view! { cx,
        {move || match is_timed_out() {
            Some(timeout) => view!{
                cx,
                <div class=BOX_CSS.to_string() + "bg-zinc-900">
                    "Please wait " {timeout} " mins"
                </div>
            }.into_view(cx),
            None => view!{
                cx,
                <div on:click=move |_| { set_modal.set(false) } class=BOX_CSS.to_string() + "bg-btn">
                    <svg viewBox="0 0 24 24" stroke-width="1.5" class="w-12 h-12 stroke-btnborder">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                    </svg>
                </div>
            }.into_view(cx)
        }}
        <CreatePCModal hidden=hidden_modal set_hidden=set_modal />
    }
}
