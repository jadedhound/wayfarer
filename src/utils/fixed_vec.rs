use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct FixedVec<T: PartialEq> {
    inner: Vec<T>,
    size: usize,
}

impl<T: PartialEq> FixedVec<T> {
    pub fn new(size: usize) -> Self {
        Self {
            inner: Vec::new(),
            size,
        }
    }
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> + '_ {
        self.inner.iter()
    }
    /// Appends a `value`; potentially removing the first
    /// entry to remain within size contrains.
    pub fn push_unique(&mut self, value: T) {
        if !self.inner.iter().any(|prev| prev == &value) {
            self.inner.push(value);
            if self.inner.len() > self.size {
                self.inner.remove(0);
            }
        }
    }
    pub fn remove_where<P>(&mut self, predicate: P) -> Option<T>
    where
        P: Fn(&T) -> bool,
    {
        let i = self.inner.iter().position(predicate)?;
        Some(self.inner.remove(i))
    }
    pub fn is_full(&self) -> bool {
        self.inner.len() >= self.size
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn resize(&mut self, size: usize) {
        self.size = size;
    }
}
