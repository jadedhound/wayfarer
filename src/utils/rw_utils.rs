use leptos::*;

use super::expect_rw;

pub trait RwUtils {
    type Item: 'static;

    /// Retrieve the provided struct.
    fn expect() -> RwSignal<Self::Item> {
        expect_rw::<Self::Item>()
    }

    /// Provide the default impl.
    fn provide() -> RwSignal<Self::Item>
    where
        Self::Item: Default,
    {
        let state = RwSignal::new(Self::Item::default());
        provide_context(state);
        state
    }

    /// Create a signal derived from a portion of this struct.
    fn slice<F, T>(f: F) -> Signal<T>
    where
        F: Fn(&Self::Item) -> T + Copy + 'static,
        T: PartialEq,
    {
        create_read_slice(Self::expect(), f)
    }

    /// Create a signal derived from a portion of this struct.
    fn rw_slice<G, S, T>(getter: G, setter: S) -> (Signal<T>, SignalSetter<T>)
    where
        G: Fn(&Self::Item) -> T + Copy + 'static,
        S: Fn(&mut Self::Item, T) + Copy + 'static,
        T: PartialEq,
    {
        create_slice(Self::expect(), getter, setter)
    }
}
