use std::rc::Rc;

use leptos::*;
use leptos_router::*;

use crate::class::*;

#[derive(Clone, Copy)]
struct IsHidden(bool);

#[component]
pub fn ClassList(cx: Scope) -> impl IntoView {
    #[allow(clippy::redundant_async_block)]
    let p_class = create_local_resource(
        cx,
        || (),
        |_| async move { fetch::<AllClasses>("/static/classes.json".into()).await },
    );
    let (is_hidden, set_hidden) = create_signal(cx, IsHidden(true));
    provide_context(cx, set_hidden);
    let list_classes = move || {
        let mut c = "z-10 absolute bg-zinc-950 h-full w-full".to_string();
        if is_hidden.get().0 {
            c.push_str(" hidden")
        };
        c
    };

    view! {
        cx,
        {move || p_class.read(cx).blank_or(cx, |data| {
            match data {
                Ok(data) => {
                    provide_context(cx, data);
                    view!{ cx,
                        <div class=list_classes >
                            <RenderList />
                        </div>
                        <div class= "w-full h-full">
                            <Outlet />
                            <ReturnFAB />
                        </div>
                    }.into_view(cx)
                },
                Err(e) => {
                    let reason = e.to_string();
                    view!{ cx, <FatalError code= "400" reason=&reason /> }.into_view(cx)
                }
            }
        })}
    }
}

#[component]
fn RenderList(cx: Scope) -> impl IntoView {
    let data = get_provided::<AllClasses>(cx);
    let names: Vec<_> = data
        .into_keys()
        .map(|name| {
            view! { cx, <ClassCard name=name /> }
        })
        .collect();
    view! { cx,
        <div class= "flex flex-col items-center justify-center h-full">
            <h2 class= "mb-4"> "Classes" </h2>
            <div class= "flex flex-col space-y-2">
                {names}
            </div>
        </div>
    }
}

#[component]
fn ClassCard(cx: Scope, name: String) -> impl IntoView {
    let hide = get_provided::<WriteSignal<IsHidden>>(cx);

    view! { cx,
        <A href=name.clone() on:click=move |_| hide.update(|a| *a = IsHidden(true))>
            <div class="bg-sky-800 p-2 rounded w-40 text-center">
                {name}
            </div>
        </A>
    }
}

#[component]
fn ReturnFAB(cx: Scope) -> impl IntoView {
    const CSS: &str =
        "absolute bottom-4 right-4 stroke-zinc-200 h-12 w-12 p-1 rounded-full bg-amber-900";
    let show = get_provided::<WriteSignal<IsHidden>>(cx);

    view! {
        cx,
        <button class=CSS on:click=move |_| show.update(|a| *a = IsHidden(false))>
            <svg viewBox="0 0 24 24" strokeWidth={1.5}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
            </svg>
        </button>
    }
}
