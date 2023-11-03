use std::future::Future;

use leptos::*;

pub mod counter;
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
pub fn add_operator(x: i32) -> String {
    if x > -1 {
        format!("+{x}")
    } else {
        x.to_string()
    }
}

/// A convenience wrapper around create_local_resource with only loads a resource once.
pub fn fetch<T, F, Fu>(asset: F) -> Resource<(), T>
where
    T: 'static,
    F: Fn() -> Fu + 'static,
    Fu: Future<Output = T> + 'static,
{
    create_local_resource_with_initial_value(|| (), move |_| asset(), None)
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

pub const fn array_len<T>(arr: &[&[T]]) -> usize {
    const fn len<T>(arr: &[&[T]], acc: usize, i: usize) -> usize {
        if i < arr.len() {
            len(arr, acc + arr[i].len(), i + 1)
        } else {
            acc
        }
    }

    len(arr, 0, 0)
}

/// Recursively flatten an array of arrays.
pub const fn flatten_array<'a, T: Copy, const COUNT: usize>(
    arr: &'a [&'a [T]],
    default: T,
) -> [T; COUNT] {
    /// Recursively flatten an array of arrays.
    const fn flatten<'a, T: Copy, const COUNT: usize>(
        arr: &'a [&'a [T]],
        arr_i: usize,
        acc: [T; COUNT],
        acc_i: usize,
    ) -> [T; COUNT] {
        if arr_i < arr.len() {
            let (acc, acc_i) = copy_subarray_to_acc(arr[arr_i], 0, acc, acc_i);
            flatten(arr, arr_i + 1, acc, acc_i)
        } else {
            acc
        }
    }

    flatten(arr, 0, [default; COUNT], 0)
}

/// Copy each subarray item into the accumulator.
const fn copy_subarray_to_acc<T: Copy, const COUNT: usize>(
    arr: &[T],
    arr_i: usize,
    mut acc: [T; COUNT],
    acc_i: usize,
) -> ([T; COUNT], usize) {
    if arr_i < arr.len() {
        acc[acc_i + arr_i] = arr[arr_i];
        copy_subarray_to_acc(arr, arr_i + 1, acc, acc_i)
    } else {
        // Adjust the current position of the `acc_i` by the amount of
        // items we just added.
        (acc, acc_i + arr_i)
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
