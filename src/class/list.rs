use leptos::*;
use leptos_router::*;

use crate::class::*;

#[derive(Clone, Copy)]
struct IsHidden(bool);

fn add_hidden<F>(f: F, mut c: String) -> String
where
    F: Fn() -> bool,
{
    if f() {
        c.push_str(" hidden");
    }
    c
}

#[component]
pub fn ClassList(cx: Scope) -> impl IntoView {
    #[allow(clippy::redundant_async_block)]
    let p_class = create_local_resource(
        cx,
        || (),
        |_| async move { fetch::<AllClasses>("classes.json".into()).await },
    );
    let (bool_wrap, set_hidden) = create_signal(cx, IsHidden(true));
    let is_hidden = move || bool_wrap.get().0;
    provide_context(cx, set_hidden);

    view! {
        cx,
        {move || p_class.read(cx).blank_or(cx, |data| {
            match data {
                Ok(data) => {
                    provide_context(cx, data);
                    view!{ cx,
                        <div class=add_hidden(is_hidden, "z-10 w-full h-full".into())>
                            <Outlet />
                            <ReturnFAB />
                        </div>
                        <div class= "fixed bg-zinc-950 h-full w-full">
                            <RenderList />
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
    let show_layer = move |_| hide.update(|a| *a = IsHidden(false));

    view! { cx,
        <A href=name.clone() on:click=show_layer>
            <div class="bg-sky-800 p-2 rounded w-40 text-center">
                {name}
            </div>
        </A>
    }
}

#[component]
fn ReturnFAB(cx: Scope) -> impl IntoView {
    const CSS: &str =
        "fixed bottom-4 right-4 stroke-zinc-200 h-12 w-12 p-1 rounded-full bg-amber-900";
    let show = get_provided::<WriteSignal<IsHidden>>(cx);

    view! {
        cx,
        <button class=CSS on:click=move |_| show.update(|a| *a = IsHidden(true))>
            <svg viewBox="0 0 24 24" strokeWidth={1.5}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
            </svg>
        </button>
    }
}
