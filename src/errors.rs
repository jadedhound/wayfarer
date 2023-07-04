use leptos::*;

#[derive(Clone)]
pub struct FatalErr {
    source: String,
    desc: String,
}

impl FatalErr {
    pub fn report<T, E>(cx: Scope, source: T, desc: E)
    where
        T: std::fmt::Display,
        E: std::fmt::Display,
    {
        let err = Self {
            source: source.to_string(),
            desc: desc.to_string(),
        };
        provide_context(cx, err);
    }
}

impl Default for FatalErr {
    fn default() -> Self {
        Self {
            source: "use_context".into(),
            desc: "unable to find fatal error".into(),
        }
    }
}

#[component]
pub fn FatalPg(cx: Scope) -> impl IntoView {
    let err = use_context::<FatalErr>(cx).unwrap_or_default();
    view! { cx,
        <div class="flex flex-col items-center justify-center h-cover space-y-4 text-center px-4">
            <h1 class= "text-red-900"> "Fatal" </h1>
            <h3> { format!("{}: {}", err.source, err.desc) } </h3>
        </div>
    }
}

#[component]
pub fn FatalError<T>(cx: Scope, code: T, reason: T) -> impl IntoView
where
    T: std::fmt::Display,
{
    view! { cx,
        <div class="flex flex-col items-center justify-center h-cover space-y-4 text-center px-4">
            <h1> {code.to_string()} </h1>
            <h3> {reason.to_string()} </h3>
        </div>
    }
}

#[component]
pub fn ComingSoon(cx: Scope) -> impl IntoView {
    view! { cx,
        <FatalError code= "TBC" reason= "This page hasn't been created yet, but it's coming!"/>
    }
}

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <FatalError code= "404" reason= "Page not found"/>
    }
}
