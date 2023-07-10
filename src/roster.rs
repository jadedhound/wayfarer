use leptos::{ev::MouseEvent, *};
use leptos_router::*;

use crate::state::{PCState, PChar};

#[component]
pub fn Roster(cx: Scope) -> impl IntoView {
    let read_pcs = use_context::<ReadSignal<PCState>>(cx).unwrap();
    let pc_list = move || {
        read_pcs.get().0.into_iter().map(|pc| {
            view!{ cx,
                <button class= "rounded bg-btn border-btnborder border-2 aspect-square flex items-center justify-center">
                    <div> {pc.name} </div>
                </button>
            }
        }).collect_view(cx)
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
            <CreatePC />
            {move || pc_list()}
        </div>
    }
}

fn create_pc(cx: Scope, _ev: MouseEvent) {
    let mut p = use_context::<ReadSignal<PCState>>(cx).unwrap().get();
    p.0.push(PChar::new());
    let set_p = use_context::<WriteSignal<PCState>>(cx).unwrap();
    set_p.set(p);
}

#[component]
fn CreatePC(cx: Scope) -> impl IntoView {
    view! { cx,
        <button on:click=move |ev| create_pc(cx, ev) class= "rounded bg-btn border-btnborder border-2 aspect-square flex items-center justify-center">
            <svg viewBox="0 0 24 24" stroke-width="1.5" class="w-12 h-12 stroke-btnborder">
              <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
            </svg>
        </button>
    }
}
