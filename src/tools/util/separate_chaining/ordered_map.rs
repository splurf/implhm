use {
    super::{Entry, EntryMut, EntrySet, RawEntry},
    crate::{Map, MapMut, MapUtil},
    std::{mem::replace, slice::Iter, vec::IntoIter},
};

#[derive(Debug)]
pub struct OrderedMap<K, V>(Vec<RawEntry<K, V>>);

impl<K, V> OrderedMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<K: Ord, V> OrderedMap<K, V> {
    fn locate(&self, key: &K) -> Result<usize, usize> {
        self.0.binary_search_by_key(&key, |f| f.key())
    }
}

impl<K, V> MapUtil<K, V> for OrderedMap<K, V> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn keys(&self) -> Iter<K> {
        todo!()
    }

    fn values(&self) -> Iter<V> {
        todo!()
    }
}

impl<K: Ord, V> Map<K, V> for OrderedMap<K, V> {
    fn get(&self, key: K) -> Option<&V> {
        if let Ok(i) = self.locate(&key) {
            Some(self.0[i].value())
        } else {
            None
        }
    }
}

impl<K: Ord, V> MapMut<K, V> for OrderedMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.locate(&key) {
            Ok(i) => Some(replace(self.0[i].value_mut(), value)),
            Err(i) => {
                self.0.insert(i, RawEntry::new(key, value));
                None
            }
        }
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let i = self.locate(&key).ok()?;
        Some(self.0.remove(i).into_value())
    }
}

impl<K, V> IntoIterator for OrderedMap<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(EntrySet::into_set)
            .collect::<Vec<(K, V)>>()
            .into_iter()
    }
}
