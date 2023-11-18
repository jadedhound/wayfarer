use std::cmp;

use serde::{Deserialize, Serialize};

/// Holds an array of elements where the indexes of remaining
/// elements don't change when items are removed or added.
/// Basically can be used as a simple ID system.
///
/// Adding and removing items are O(1).
/// Removed indexes are reused and thus won't be exausted with
/// use.
/// Note: Added elements won't always be pushed to the end of the
/// array. Thus position of the new element isn't reliable.
#[derive(Serialize, Deserialize, Clone)]
pub struct IndexMap<T: Clone> {
    removed_ids: Vec<usize>,
    values: Vec<Option<T>>,
}

impl<T: Clone> Default for IndexMap<T> {
    fn default() -> Self {
        Self {
            removed_ids: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl<T: Clone> From<Vec<T>> for IndexMap<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            values: value.into_iter().map(|x| Some(x)).collect(),
            ..Default::default()
        }
    }
}

impl<T: Clone> From<Vec<(usize, T)>> for IndexMap<T> {
    fn from(value: Vec<(usize, T)>) -> Self {
        let max_id = value.iter().fold(0, |acc, (id, _)| cmp::max(acc, *id));
        let mut result = vec![None; max_id + 1];
        for (id, ele) in value {
            result[id] = Some(ele)
        }
        let mut removed_ids = vec![];
        for (id, ele) in result.iter().enumerate() {
            if ele.is_none() {
                removed_ids.push(id)
            }
        }
        Self {
            values: result,
            removed_ids,
        }
    }
}

impl<A: Clone> FromIterator<A> for IndexMap<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self {
            values: iter.into_iter().map(|x| Some(x)).collect(),
            ..Default::default()
        }
    }
}

impl<T: Clone> IndexMap<T> {
    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.values.iter().flatten()
    }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.values.iter_mut().flatten()
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, x)| Some((i, x.as_ref()?)))
    }
    pub fn keys(&self) -> impl Iterator<Item = usize> + '_ {
        self.iter().map(|(i, _)| i)
    }
    pub fn add(&mut self, t: T) -> usize {
        if let Some(id) = self.removed_ids.pop() {
            self.values[id] = Some(t);
            id
        } else {
            self.values.push(Some(t));
            self.len() - 1
        }
    }
    pub fn remove(&mut self, id: usize) -> Option<T> {
        let val = self.values.get(id)?.clone();
        self.values[id] = None;
        self.removed_ids.push(id);
        val
    }
    pub fn get(&self, id: usize) -> Option<&T> {
        self.values.get(id).and_then(|x| x.as_ref())
    }
    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.values.get_mut(id).and_then(|x| x.as_mut())
    }
    pub fn expect(&self, id: usize) -> T
    where
        T: Default,
    {
        self.get(id).cloned().unwrap_or_default()
    }
    pub fn len(&self) -> usize {
        self.values.len() - self.removed_ids.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn position<P>(&self, p: P) -> Option<usize>
    where
        P: Fn(&T) -> bool,
    {
        self.iter().find(|(_, item)| p(item)).map(|(id, _)| id)
    }
}
