use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FixedVec<T> {
    inner: Vec<T>,
    max_len: usize,
}

impl<T> FixedVec<T> {
    pub fn new(max_len: usize) -> Self {
        Self {
            inner: Vec::new(),
            max_len,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.inner.iter()
    }
    pub fn push(&mut self, value: T) {
        if self.inner.len() < self.max_len {
            self.inner.push(value)
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
        self.inner.len() >= self.max_len
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
