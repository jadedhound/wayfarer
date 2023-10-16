use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EnumArray<E, const N: usize>
where
    E: EnumRef + IntoEnumIterator + Copy,
{
    #[serde(with = "serde_arrays")]
    arr: [i32; N],
    _marker: PhantomData<E>,
}

impl<E, const N: usize> EnumArray<E, N>
where
    E: EnumRef + IntoEnumIterator + Copy,
{
    fn new(arr: [i32; N]) -> Self {
        Self {
            arr,
            _marker: PhantomData,
        }
    }
    pub fn iter_enum(&self) -> impl Iterator<Item = (E, i32)> + '_ {
        E::iter().map(|e| (e, self.get(e)))
    }
    pub fn get(&self, id: E) -> i32 {
        self.arr[id.index()]
    }
    pub fn get_mut(&mut self, id: E) -> &mut i32 {
        &mut self.arr[id.index()]
    }
}

impl<E, const N: usize> Default for EnumArray<E, N>
where
    E: EnumRef + IntoEnumIterator + Copy,
{
    fn default() -> Self {
        let arr = [(); N].map(|_| i32::default());
        Self::new(arr)
    }
}

impl<E, const N: usize> From<Vec<(E, i32)>> for EnumArray<E, N>
where
    E: EnumRef + IntoEnumIterator + Copy,
{
    fn from(value: Vec<(E, i32)>) -> Self {
        let mut arr = Self::default();
        for (e, t) in value {
            *arr.get_mut(e) = t
        }
        arr
    }
}

pub trait EnumRef {
    fn index(&self) -> usize;
}
