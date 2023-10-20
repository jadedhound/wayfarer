use std::cell::RefCell;
use std::rc::Rc;

use leptos::{expect_context, provide_context};
use nanorand::{Rng, WyRand};

#[derive(Clone)]
pub struct Rand {
    inner: WyRand,
}

impl Rand {
    /// Provide `Rand` for use anywhere in the app.
    pub fn provide() {
        let seed = (js_sys::Math::random() * 10_f64.powf(10.0)) as u64;
        let cell = RefCell::new(Rand {
            inner: WyRand::new_seed(seed),
        });
        provide_context(Rc::new(cell));
    }

    /// Recall `Rand` from context and use it.
    pub fn with<F, T>(f: F) -> T
    where
        F: FnOnce(&mut Rand) -> T,
    {
        let cell = expect_context::<Rc<RefCell<Rand>>>();
        let mut cell = cell.as_ref().borrow_mut();
        f(&mut cell)
    }

    /// Picks an element from a given array
    pub fn pick<T: Copy>(&mut self, arr: &[T]) -> T {
        let i = self.inner.generate_range(0_usize..arr.len());
        arr[i]
    }

    /// Random number `from` to `to`.
    pub fn range(&mut self, from: u32, to: u32) -> u32 {
        self.inner.generate_range(from..=to)
    }
}
