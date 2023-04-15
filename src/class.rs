use leptos::*;
use leptos_router::*;
use std::collections::HashMap;

use serde::Deserialize;

use crate::errors::*;
use crate::utils::*;

type Features = HashMap<String, String>;
type AllClasses = HashMap<String, PClass>;

#[derive(Deserialize, Clone)]
struct PClass {
    desc: String,
    adv_table: [String; 4],
    basics: PCBasics,
    equipment: Vec<String>,
    core: Option<Features>,
    archetypes: HashMap<String, Archetype>,
}

#[derive(Deserialize, Clone)]
struct Archetype {
    prof: String,
    features: Features,
}

#[derive(Deserialize, Clone)]
struct PCBasics {
    starting_hp: u8,
    level_hp: u8,
    armour_prof: String,
    weap_prof: String,
}

#[component]
pub fn ClassList(cx: Scope) -> impl IntoView {
    #[allow(clippy::redundant_async_block)]
    let p_class = create_local_resource(
        cx,
        || (),
        |_| async move { fetch::<AllClasses>("/static/classes.json".into()).await },
    );
    let (is_hidden, set_hidden) = create_signal(cx, false);
    provide_context(cx, set_hidden);
    let list_classes = move || {
        let mut c = "z-10 absolute bg-zinc-950 h-full w-full".to_string();
        if is_hidden.get() {
            c.push_str(" hidden")
        };
        c
    };

    view! {
        cx,
        <div class=list_classes >
        {move || p_class.read(cx).blank_or(cx, |data| {
            match data {
                Ok(data) => {
                    provide_context(cx, data);
                    view!{ cx, <RenderList /> }.into_view(cx)
                },
                Err(e) => {
                    let reason = e.to_string();
                    view!{ cx, <FatalError code= "400" reason=&reason /> }.into_view(cx)
                }
            }
        })}
        </div>
        <Outlet />
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
    let hide = get_provided::<WriteSignal<bool>>(cx);

    view! { cx,
        <A href=name.clone() on:click=move |_| hide.update(|v| *v = true)>
            <div class="bg-sky-800 p-2 rounded w-40 text-center">
                {name}
            </div>
        </A>
    }
}

#[component]
pub fn ClassDetails(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="z-0">
        "Class Details"
        </div>
    }
}

#[component]
pub fn ClassEmptyDetails(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
        "Nothing to see here"
        </div>
    }
}
