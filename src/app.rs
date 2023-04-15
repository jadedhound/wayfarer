use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <AppRouter />
    }
}

#[component]
fn AppRouter(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path= "" view=move |cx| view! { cx, <Home/> }/>
                <Route path= "/coming-soon" view=|cx| view! { cx, <ComingSoon /> }/>
                <Route path= "/*any" view=|cx| view! { cx, <NotFound/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col h-full items-center justify-center text-center">
            <h1> "Wayfarer" </h1>
            <div class= "grid my-8 divide-y-2 divide-amber-600 w-fit">
                <div class= "flex flex-col space-y-2 items-center pb-2">
                    <Card title= "Create Character" link= "/coming-soon" />
                    <Card title= "FAQ" link= "/coming-soon" />
                </div>
                <div class= "flex flex-col space-y-2 items-center pt-2">
                    <Card title= "Spellbook" link= "/coming-soon" />
                    <Card title= "Combat" link= "/coming-soon" />
                    <Card title= "Adventuring" link= "/coming-soon" />
                    <Card title= "Spellcasting" link= "/coming-soon" />
                </div>
            </div>
        </div>
    }
}

#[component]
fn Card<T>(cx: Scope, title: T, link: T) -> impl IntoView
where
    T: AsRef<str>,
{
    view! { cx,
        <a href=link.as_ref()>
            <div class="p-2 bg-sky-800 rounded w-40">
                 {title.as_ref().to_string()}
            </div>
        </a>
    }
}

#[component]
fn FatalError<T>(cx: Scope, code: T, reason: T) -> impl IntoView
where
    T: AsRef<str>,
{
    view! { cx,
        <div class="flex flex-col items-center justify-center h-full space-y-4 text-center px-4">
            <h1> {code.as_ref().to_string()} </h1>
            <h3> {reason.as_ref().to_string()} </h3>
        </div>
    }
}

#[component]
fn ComingSoon(cx: Scope) -> impl IntoView {
    view! { cx,
        <FatalError code= "TBC" reason= "This page hasn't been created yet, but it's coming!"/>
    }
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <FatalError code= "404" reason= "Page not found"/>
    }
}
