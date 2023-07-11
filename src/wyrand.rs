use std::{cell::Cell, rc::Rc};

use leptos::{use_context, Scope};

const P0: u64 = 0xa076_1d64_78bd_642f;
const P1: u64 = 0xe703_7ed1_a0b4_28db;

#[derive(Copy, Clone)]
pub struct WyRand {
    pub seed: u64,
}

impl WyRand {
    pub fn new() -> Self {
        let seed = (js_sys::Math::random() * 10_f64.powf(10.0)) as u64;
        Self { seed }
    }

    pub fn from_context(cx: Scope) -> Self {
        let cell = use_context::<Rc<Cell<WyRand>>>(cx).unwrap();
        cell.get()
    }

    pub fn to_context(mut self, cx: Scope) {
        let cell = use_context::<Rc<Cell<WyRand>>>(cx).unwrap();
        cell.set(self)
    }

    pub fn rand(&mut self) -> u64 {
        log::info!("seed: {}", self.seed);
        self.seed = self.seed.wrapping_add(P0);
        let r = u128::from(self.seed) * u128::from(self.seed ^ P1);
        ((r >> 64) ^ r) as u64
    }

    pub fn range(&mut self, max: u64) -> u64 {
        self.rand() % max
    }

    pub fn from_arr<T: Clone>(&mut self, arr: &[T]) -> T {
        arr[self.range(arr.len() as u64 - 1) as usize].clone()
    }

    pub fn dice(&mut self, num: u64, faces: u64) -> Vec<u64> {
        (0..num).map(|_| self.range(faces)).collect()
    }
}
