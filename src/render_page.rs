use leptos::*;

use crate::{
    errors::*,
    utils::{fetch_text, ToView},
};

#[component]
pub fn RenderPage(cx: Scope, page: &'static str) -> impl IntoView {
    #[allow(clippy::redundant_async_block)]
    let html = create_local_resource(
        cx,
        || (),
        move |_| async move { fetch_text(format!("pages/{page}.html")).await },
    );

    view! { cx,
        {move || html.read(cx).blank_or(cx, |data| {
            match data {
                Ok(page) => {
                    view!{ cx,
                        <div class= "px-4 h-full w-full" inner_html=page></div>
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
