use gloo::storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;

use crate::class::{details::*, list::*};
use crate::errors::*;
use crate::home::*;
use crate::login::*;
use crate::render_page::*;
use crate::spellbook::*;

#[component]
fn MainRouter(cx: Scope) -> impl IntoView {
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
                <Route path= "/spellbook" view=|cx| view! { cx, <Spellbook /> }/>
                <Route path= "/spellbook/:school" view=|cx| view! { cx, <Spellbook /> }/>
                <Route path= "/combat" view=move |cx| view! { cx, <RenderPage page="combat" /> }/>
                <Route path= "/adventuring" view=move |cx| view! { cx, <RenderPage page="adventuring" /> }/>
                <Route path= "/spellcasting" view=move |cx| view! { cx, <RenderPage page="spellcasting" /> }/>
                <Route path= "/coming-soon" view=|cx| view! { cx, <ComingSoon /> }/>
                <Route path= "/*any" view=|cx| view! { cx, <NotFound/> }/>
            </Routes>
        </Router>
    }
}

#[derive(Clone)]
pub struct LoginStatus(Option<Login>);

#[component]
pub fn RouterScout(cx: Scope) -> impl IntoView {
    let status = LoginStatus(LocalStorage::get::<Login>("login").ok());
    let (get_login, set_login) = create_signal(cx, status);
    provide_context(cx, set_login);
    view! {
        cx,
        {match get_login.get().0 {
            Some(login) => {
                provide_context(cx, login);
                view!{ cx, <MainRouter /> }.into_view(cx)
            },
            None => {
                view!{ cx, <LoginPg /> }.into_view(cx)
            }
        }}
    }
}
