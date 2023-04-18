use leptos::*;

use crate::{
    errors::*,
    utils::{fetch_text, ToView},
};

#[component]
pub fn CharCreation(cx: Scope) -> impl IntoView {
    #[allow(clippy::redundant_async_block)]
    let html = create_local_resource(
        cx,
        || (),
        |_| async move { fetch_text("pages/creation_guide.html".into()).await },
    );
    view! { cx,
        {move || html.read(cx).blank_or(cx, |data| {
            match data {
                Ok(page) => {
                    view!{ cx,
                        <div class= "px-4">
                            <h2> "Character Creation Guide" </h2>
                            <div class= "" inner_html=page></div>
                        </div>
                    }.into_view(cx)
                },
                Err(e) => {
                    let reason = e.to_string();
                    view!{ cx, <FatalError code= "400" reason=&reason /> }.into_view(cx)
                }
            }
        })}
    }
}
