use leptos::*;

pub mod counter;
pub mod db;
pub mod enum_array;
pub mod fixed_vec;
pub mod index_map;
pub mod rw_utils;
pub mod search;
pub mod turns;

/// A long string used to test layout resilience.
#[allow(dead_code)]
pub const LONG_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque erat nulla, porttitor non purus eu, sollicitudin ornare dui. Praesent pulvinar scelerisque bibendum. Maecenas lobortis viverra venenatis. Donec congue in augue id viverra. Proin pulvinar, odio sit amet euismod tristique, neque ipsum blandit tortor, varius iaculis ante diam at erat. Praesent erat tellus, imperdiet at est quis, malesuada lacinia lorem. Duis et pharetra odio, efficitur posuere justo. Curabitur quam augue, imperdiet cursus vestibulum quis, vulputate a tellus. Vivamus sit amet nibh non eros molestie aliquet.";

// -----------------------------------
// SIMPLE FUNCTIONS
// -----------------------------------

/// Get a read/write signal that has already been provided.
pub fn expect_rw<T>() -> RwSignal<T> {
    expect_context::<RwSignal<T>>()
}

/// Adds a `+` to positive values.
pub fn split_operator(x: i32) -> (char, i32) {
    if x > -1 {
        ('+', x)
    } else {
        ('-', -x)
    }
}

/// Concats the string (with a space inbetween) if `predicate` is true.
pub fn concat_if<F, S>(predicate: F, base: S, addon: S) -> impl Fn() -> String
where
    S: std::fmt::Display,
    F: Fn() -> bool,
{
    move || {
        if predicate() {
            format!("{base} {addon}")
        } else {
            base.to_string()
        }
    }
}

pub trait ArrayEnhance {
    fn is_not_empty(&self) -> bool;
}

impl<T> ArrayEnhance for Vec<T> {
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}
impl ArrayEnhance for String {
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

pub trait RwSignalEnhance<T> {
    /// An update fn that allows for closures that return a value, thus
    /// making one-line updates possible.
    fn update_discard<A>(&self, f: impl FnOnce(&mut T) -> A);
    /// Reset a given signal to its default state.
    fn reset(&self)
    where
        T: Default;
}

impl<T> RwSignalEnhance<T> for RwSignal<T> {
    fn update_discard<A>(&self, f: impl FnOnce(&mut T) -> A) {
        self.update(|x| {
            f(x);
        })
    }

    fn reset(&self)
    where
        T: Default,
    {
        self.set(T::default())
    }
}
