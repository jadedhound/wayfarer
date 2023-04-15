use leptos::*;

#[component]
pub fn FatalError<T>(cx: Scope, code: T, reason: T) -> impl IntoView
where
    T: std::fmt::Display,
{
    view! { cx,
        <div class="flex flex-col items-center justify-center h-full space-y-4 text-center px-4">
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
