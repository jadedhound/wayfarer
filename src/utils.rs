use gloo::net::Error as GlooError;
use leptos::*;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum UtilsErr {
    #[error("Failed to fetch: {0}")]
    FetchFail(String),
}

/// Fetch an asset from the static folder as JSON.
pub async fn fetch<T>(url: String) -> Result<T, UtilsErr>
where
    T: DeserializeOwned,
{
    let to_utils_error = |e: GlooError| UtilsErr::FetchFail(e.to_string());
    gloo::net::http::Request::new(&format!("/static/{url}"))
        .send()
        .await
        .map_err(to_utils_error)?
        .json()
        .await
        .map_err(to_utils_error)
}

/// Fetch an asset from the static folder as `String`.
pub async fn fetch_text(url: String) -> Result<String, UtilsErr> {
    let to_utils_error = |e: GlooError| UtilsErr::FetchFail(e.to_string());
    gloo::net::http::Request::new(&format!("/static/{url}"))
        .send()
        .await
        .map_err(to_utils_error)?
        .text()
        .await
        .map_err(to_utils_error)
}

#[inline]
/// Get an item from context that is guaranteed to have been provided.
/// Recommended instead of simply unwrapping.
/// See: `https://rustwasm.github.io/book/reference/code-size.html` for more info.
pub fn get_provided<T>(cx: Scope) -> T
where
    T: Clone + 'static,
{
    use std::process;
    match use_context::<T>(cx) {
        Some(t) => t,
        None => process::abort(),
    }
}

pub trait ToView<T> {
    fn blank_or<F>(self, cx: Scope, f: F) -> View
    where
        F: Fn(T) -> View;
}

impl<T> ToView<T> for Option<T> {
    fn blank_or<F>(self, cx: Scope, f: F) -> View
    where
        F: Fn(T) -> View,
    {
        match self {
            Some(t) => f(t),
            None => view! { cx, }.into_view(cx),
        }
    }
}