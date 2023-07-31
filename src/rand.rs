use std::cell::RefCell;
use std::rc::Rc;

use leptos::{provide_context, use_context, Scope};
use nanorand::{Rng, WyRand};

/// Simplifies getting a `&mut Rand` struct
pub fn rand_context<T>(cx: Scope, f: impl FnOnce(&mut Rand) -> T) -> T {
    let cell = use_context::<Rc<RefCell<Rand>>>(cx).unwrap();
    let mut rand = cell.as_ref().borrow_mut();
    f(&mut rand)
}

pub fn init_rand(cx: Scope) {
    let seed = (js_sys::Math::random() * 10_f64.powf(10.0)) as u64;
    let cell = RefCell::new(Rand {
        inner: WyRand::new_seed(seed),
    });
    provide_context(cx, Rc::new(cell));
}

#[derive(Clone)]
pub struct Rand {
    inner: nanorand::WyRand,
}

impl Rand {
    /// Picks an element from a given array
    pub fn pick<T: Copy>(&mut self, arr: &[T]) -> T {
        let i = self.inner.generate_range(0_usize..arr.len());
        arr[i]
    }

    /// Random number `from` to `to`.
    pub fn range(&mut self, from: u32, to: u32) -> u32 {
        self.inner.generate_range(from..to)
    }
}
