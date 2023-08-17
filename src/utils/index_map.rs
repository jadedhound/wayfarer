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

impl<T: Clone> IndexMap<T> {
    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.values.iter().flatten()
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, x)| Some((i, x.as_ref()?)))
    }
    pub fn clone_iter(&self) -> impl Iterator<Item = (usize, T)> + '_ {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, x)| Some((i, x.clone()?)))
    }
    pub fn add(&mut self, t: T) {
        if let Some(id) = self.removed_ids.pop() {
            self.values[id] = Some(t)
        } else {
            self.values.push(Some(t))
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
    pub fn len(&self) -> usize {
        self.values.len() - self.removed_ids.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn clone_map<A, F>(&self, mut f: F) -> IndexMap<A>
    where
        A: Clone,
        F: FnMut(&T) -> A,
    {
        let values = self.values.iter().map(|x| x.as_ref().map(&mut f)).collect();
        IndexMap {
            removed_ids: self.removed_ids.clone(),
            values,
        }
    }
}
