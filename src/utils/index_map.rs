use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct IndexMap<T>
where
    T: Clone,
{
    inner: HashMap<usize, T>,
}

impl<T> Default for IndexMap<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl<T> IndexMap<T>
where
    T: Clone,
{
    pub fn new(t: Vec<T>) -> Self {
        let inner = t.into_iter().enumerate().collect();
        Self { inner }
    }
    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.inner.values()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&usize, &T)> + '_ {
        self.inner.iter()
    }
    pub fn push(&mut self, t: T) {
        let i = self.inner.keys().last().map(|i| *i + 1).unwrap_or(0);
        self.inner.insert(i, t);
    }
    pub fn remove(&mut self, id: &usize) -> Option<T> {
        self.inner.remove(id)
    }
    pub fn get(&self, id: &usize) -> Option<&T> {
        self.inner.get(id)
    }
    pub fn get_mut(&mut self, id: &usize) -> Option<&mut T> {
        self.inner.get_mut(id)
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
