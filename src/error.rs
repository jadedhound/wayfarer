use leptos::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("page not found")]
    NotFound,
    #[error(transparent)]
    SimpleIndex(#[from] simple_index::Error),
}

#[derive(Clone)]
pub struct FatalErr(String, String);

impl FatalErr {
    pub fn provide<T>(cx: Scope, origin: T, err: Error)
    where
        T: std::fmt::Display,
    {
        provide_context(cx, FatalErr(origin.to_string(), err.to_string()))
    }
}

#[component]
pub fn FatalPg(cx: Scope) -> impl IntoView {
    let e = use_context::<FatalErr>(cx).unwrap();
    view! { cx,
        <div class="flex flex-col items-center justify-center h-cover space-y-4 text-center px-4">
            <h1 class= "text-red-800"> "Fatal" </h1>
            <h3> {e.0} </h3>
            <h3> {e.1} </h3>
        </div>
    }
}

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    FatalErr::provide(cx, "router", Error::NotFound);
    view! { cx,
        <FatalPg />
    }
}