use leptos::*;

pub mod db;
pub mod index_map;
pub mod time;

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
}
