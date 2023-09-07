use leptos::*;

pub mod counter;
pub mod db;
pub mod index_map;
pub mod search;
pub mod turns;

/// A long string used to text layout resilience.
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

pub fn some_if(predicate: bool) -> Option<()> {
    if predicate {
        Some(())
    } else {
        None
    }
}

// -----------------------------------
// CONTEXT HELPER TRAIT
// -----------------------------------

pub trait RwProvided {
    type Item: 'static;

    fn with<F, T>(f: F) -> T
    where
        F: FnOnce(&Self::Item) -> T,
    {
        expect_rw::<Self::Item>().with(|x| f(x))
    }
    fn untracked<F, T>(f: F) -> T
    where
        F: FnOnce(&Self::Item) -> T,
    {
        expect_rw::<Self::Item>().with_untracked(|x| f(x))
    }

    fn update<F, T>(f: F)
    where
        F: FnOnce(&mut Self::Item) -> T,
    {
        expect_rw::<Self::Item>().update(|sesh| {
            f(sesh);
        })
    }

    fn slice<F, T>(f: F) -> leptos::Signal<T>
    where
        F: Fn(&Self::Item) -> T + Copy + 'static,
        T: PartialEq,
    {
        create_read_slice(expect_rw::<Self::Item>(), f)
    }
}
