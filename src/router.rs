use leptos::*;
use leptos_router::*;

use crate::class::{details::*, list::*};
use crate::errors::*;
use crate::home::*;
use crate::render_page::*;

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path= "" view=move |cx| view! { cx, <Home /> }/>
                <Route path= "/creation-guide" view=move |cx| view! { cx, <RenderPage page="creation_guide" /> }/>
                <Route path= "/faq" view=move |cx| view! { cx, <RenderPage page="faq" /> }/>
                <Route path= "/class" view=move |cx| view! { cx, <ClassList /> }>
                    <Route path= "" view=move |cx| view! { cx, <NoClassDetails /> } />
                    <Route path= ":name" view=move |cx| view! { cx, <ClassDetails /> }/>
                </Route>
                <Route path= "/combat" view=move |cx| view! { cx, <RenderPage page="combat" /> }/>
                <Route path= "/adventuring" view=move |cx| view! { cx, <RenderPage page="adventuring" /> }/>
                <Route path= "/spellcasting" view=move |cx| view! { cx, <RenderPage page="spellcasting" /> }/>
                <Route path= "/coming-soon" view=|cx| view! { cx, <ComingSoon /> }/>
                <Route path= "/*any" view=|cx| view! { cx, <NotFound/> }/>
            </Routes>
        </Router>
    }
}
