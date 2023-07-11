use std::{cell::Cell, rc::Rc};

use leptos::*;

use crate::wyrand::WyRand;

pub fn read_context<T>(cx: Scope) -> ReadSignal<T> {
    use_context::<ReadSignal<T>>(cx).unwrap()
}

pub fn write_context<T>(cx: Scope) -> WriteSignal<T> {
    use_context::<WriteSignal<T>>(cx).unwrap()
}

pub fn wyrand_context<T>(cx: Scope, f: impl FnOnce(&mut WyRand) -> T) -> T {
    let cell = use_context::<Rc<Cell<WyRand>>>(cx).unwrap();
    let mut wyrand = cell.get();
    let res = f(&mut wyrand);
    cell.set(wyrand);
    res
}
