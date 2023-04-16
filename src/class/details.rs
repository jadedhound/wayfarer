use leptos::*;
use leptos_router::*;

use crate::class::*;

#[derive(Clone)]
struct ClassName(String);

#[component]
pub fn ClassDetails(cx: Scope) -> impl IntoView {
    let name = move || use_params_map(cx).get().get("name").cloned();
    let class = move || {
        name().as_ref().and_then(|name| {
            let class = get_provided::<AllClasses>(cx).get(name)?.clone();
            Some((name.clone(), class))
        })
    };
    view! {
        cx,
        {move || match class() {
            Some((name, class)) => {
                provide_context(cx, ClassName(name));
                provide_context(cx, class);
                view!{ cx, <RenderDetails /> }.into_view(cx)
            },
            None => view! {cx, <NotFound /> }.into_view(cx)
        }}
    }
}

#[component]
pub fn NoClassDetails(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class= "flex flex-col items-center justify-center h-full text-center">
            <h2 class= "mb-4"> "Select a class" </h2>
            <h4> "Use the list button to return" </h4>
        </div>
    }
}

#[component]
fn RenderDetails(cx: Scope) -> impl IntoView {
    let name = get_provided::<ClassName>(cx).0;
    let class = get_provided::<PClass>(cx);
    provide_context(cx, class.adv_table);

    view! {
        cx,
        <div class= "flex flex-col h-full px-4">
            <h1 class= "mb-4"> {name} </h1>
            {class.desc}
            <AdvTable />
        </div>
    }
}

#[component]
fn AdvTable(cx: Scope) -> impl IntoView {
    use std::process;
    let mut data = get_provided::<[String; 4]>(cx).into_iter();
    let mut feat = move || match data.next() {
        Some(t) => t,
        None => process::abort(),
    };
    let name = get_provided::<ClassName>(cx).0;
    let arche = format!("{name} archetype");
    let rows = vec![
        view! {cx, {format!("Starting HP, {name} equipment, {}", feat())} }.into_view(cx),
        view! {cx, <span class= "italic"> {&arche} </span> }.into_view(cx),
        view! {cx, {feat()} }.into_view(cx),
        view! {cx, <span class= "italic"> {&arche} </span> }.into_view(cx),
        view! {cx, {feat()} }.into_view(cx),
        view! {cx, <span class= "italic"> {&arche} </span> ", establish a guild" }.into_view(cx),
        view! {cx, "Add +1 to any ability score" }.into_view(cx),
        view! {cx, "Add +1 to any ability score" }.into_view(cx),
        view! {cx, "Add +1 to any ability score" }.into_view(cx),
    ];
    let v_rows: Vec<_> = rows
        .into_iter()
        .enumerate()
        .map(|(i, r)| {
            view! {
                cx,
                <tr>
                    <td> {i + 1} </td>
                    <td> {r} </td>
                </tr>
            }
        })
        .collect();

    view! {
        cx,
        <div class= "">
            <h4 class= "text-center mt-4"> "Advancement Table" </h4>
            <table class= "mt-2 table-shaded">
                <tr>
                    <th> "LEVEL" </th>
                    <th> "FEATURES" </th>
                </tr>
                <tbody class= "tr-px-2">
                    {v_rows}
                </tbody>
            </table>
        </div>
    }
}
