use leptos::*;
use log::info;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Login {
    pub user: String,
    pub pass: String,
}

impl Login {
    fn is_incomplete(&self) -> bool {
        self.user.is_empty() | self.pass.is_empty()
    }
}

#[derive(Error, Debug, Clone)]
enum LoginErr {
    #[error("Please ensure a user and password has been given")]
    Incomplete,
}

#[component]
pub fn LoginPg(cx: Scope) -> impl IntoView {
    const IN_BOX: &str = "font-sans rounded-xl bg-zinc-700 p-2 mt-2 invalid:outline-amber-500";
    let (login, set_login) = create_signal(cx, Login::default());
    let change_usr = move |ev| {
        set_login(Login {
            user: event_target_value(&ev),
            ..login.get()
        })
    };
    let change_pass = move |ev| {
        set_login(Login {
            pass: event_target_value(&ev),
            ..login.get()
        })
    };

    view! { cx,
        <div class="flex flex-col h-cover items-center justify-center text-center px-4">
            <h1> "Wayfarer" </h1>
            <input class=IN_BOX on:input=change_usr placeholder="User" invalid />
            <input class=IN_BOX on:input=change_pass type="password" placeholder="Password" />
            <LoginBtn login=login />
        </div>
    }
}

#[component]
fn LoginBtn(cx: Scope, login: ReadSignal<Login>) -> impl IntoView {
    let validate_login = create_local_resource(cx, login, |lgn| async move {
        info!("Got new login details: {:?}", lgn);
        if lgn.is_incomplete() {
            Some(LoginErr::Incomplete)
        } else {
            None
        }
    });
    view! {
        cx,
        <button class= "font-sans-condensed mt-4 bg-wfblue rounded-xl w-36 py-2"> "Create" </button>
        {move || match validate_login.read(cx) {
            None => view! { cx, "Loading..." }.into_view(cx),
            Some(resp) => view! {
                cx,
                {move || match &resp {
                    Some(err) => view! { cx, {err.to_string()} }.into_view(cx),
                    None => view! { cx, "All good!" }.into_view(cx),
                }}
            }.into_view(cx),
        }}
    }
}
