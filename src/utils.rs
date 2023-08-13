use leptos::*;

pub mod db;
pub mod index_map;

// -----------------------------------
// SIMPLE FUNCTIONS
// -----------------------------------

/// Get a read/write signal that has already been provided
pub fn rw_context<T>(cx: Scope) -> RwSignal<T> {
    use_context::<RwSignal<T>>(cx).unwrap()
}

/// Capitalises the first letter of a given string.
pub fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Flattens and joins a `vec` of string into a single string.
pub fn flat_concat(v: Vec<Option<String>>, join: &'static str) -> Option<String> {
    v.into_iter().flatten().reduce(|mut acc, e| {
        acc.push_str(join);
        acc.push_str(&e);
        acc
    })
}

/// Adds a `+` to positive values.
pub fn split_operator(x: i32) -> (char, i32) {
    if x > -1 {
        ('+', x)
    } else {
        ('-', -x)
    }
}
