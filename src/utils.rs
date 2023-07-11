use leptos::*;

pub fn read_context<T>(cx: Scope) -> ReadSignal<T> {
    use_context::<ReadSignal<T>>(cx).unwrap()
}

pub fn write_context<T>(cx: Scope) -> WriteSignal<T> {
    use_context::<WriteSignal<T>>(cx).unwrap()
}
