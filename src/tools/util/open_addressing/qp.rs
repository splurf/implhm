use crate::{create_table, MapMut, MapUtil};

use {
    crate::{Map, RawEntry, DEFAULT_CAPACITY},
    std::hash::Hash,
};

/** HashMap implementation handling *collision* by *double hashing* */
#[derive(Debug)]
pub struct QPHashMap<K, V>(Vec<Option<RawEntry<K, V>>>);

impl<K, V> QPHashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self(create_table(capacity, |_| None))
    }
}

impl<K, V> Map<K, V> for QPHashMap<K, V> {
    fn get(&self, key: K) -> Option<&V> {
        todo!()
    }
}

impl<K, V> MapMut<K, V> for QPHashMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!()
    }

    fn remove(&mut self, key: K) -> Option<V> {
        todo!()
    }
}

impl<K, V> MapUtil<K, V> for QPHashMap<K, V> {
    fn len(&self) -> usize {
        todo!()
    }

    fn keys(&self) -> std::slice::Iter<K> {
        todo!()
    }

    fn values(&self) -> std::slice::Iter<V> {
        todo!()
    }

    fn iter(&self) -> std::slice::Iter<RawEntry<K, V>> {
        todo!()
    }

    fn into_iter(self) -> std::vec::IntoIter<RawEntry<K, V>> {
        todo!()
    }
}

impl<K: Hash + Ord, V> Default for QPHashMap<K, V> {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}

impl<K: Hash + Ord, V> From<Vec<(K, V)>> for QPHashMap<K, V> {
    fn from(kv: Vec<(K, V)>) -> Self {
        let mut map = Self::default();
        for (key, value) in kv.into_iter() {
            map.insert(key, value);
        }
        map
    }
}
