use leptos::*;

use super::expect_rw;

pub trait RwUtils
where
    Self: Sized + 'static,
{
    /// Retrieve the provided struct.
    fn expect() -> RwSignal<Self> {
        expect_rw::<Self>()
    }

    /// Provide the default impl.
    fn provide() -> RwSignal<Self>
    where
        Self: Default,
    {
        let state = RwSignal::new(Self::default());
        provide_context(state);
        state
    }

    /// Create a signal derived from a portion of this struct.
    fn slice<F, T>(f: F) -> Signal<T>
    where
        F: Fn(&Self) -> T + Copy + 'static,
        T: PartialEq,
    {
        create_read_slice(Self::expect(), f)
    }

    /// Create a signal derived from a portion of this struct.
    fn rw_slice<G, S, T>(getter: G, setter: S) -> (Signal<T>, SignalSetter<T>)
    where
        G: Fn(&Self) -> T + Copy + 'static,
        S: Fn(&mut Self, T) + Copy + 'static,
        T: PartialEq,
    {
        create_slice(Self::expect(), getter, setter)
    }
}
