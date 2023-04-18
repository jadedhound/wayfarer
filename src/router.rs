use leptos::*;
use leptos_router::*;

use crate::char_creation::*;
use crate::class::{details::*, list::*};
use crate::errors::*;
use crate::home::*;

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path= "" view=move |cx| view! { cx, <Home /> }/>
                <Route path= "/char-creation" view=move |cx| view! { cx, <CharCreation /> }/>
                <Route path= "/class" view=move |cx| view! { cx, <ClassList /> }>
                    <Route path= "" view=move |cx| view! { cx, <NoClassDetails /> } />
                    <Route path= ":name" view=move |cx| view! { cx, <ClassDetails /> }/>
                </Route>
                <Route path= "/coming-soon" view=|cx| view! { cx, <ComingSoon /> }/>
                <Route path= "/*any" view=|cx| view! { cx, <NotFound/> }/>
            </Routes>
        </Router>
    }
}
