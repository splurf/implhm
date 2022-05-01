use {
    crate::{Entry, Map, DEFAULT_CAPACITY},
    std::hash::Hash,
};

/** HashMap implementation handling *collision* by *quadratic probing* */
#[derive(Debug)]
pub struct QPHashMap<K, V> {
    _data: Vec<Entry<K, V>>,
    _capacity: usize,
}

impl<K: Clone + Hash + Ord, V: Clone> Map<K, V> for QPHashMap<K, V> {
    fn new(_capacity: usize) -> Self {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn get(&self, _key: K) -> Option<V> {
        todo!()
    }

    fn insert(&mut self, _key: K, _value: V) -> Option<V> {
        todo!()
    }

    fn remove(&mut self, _key: K) -> Option<V> {
        todo!()
    }

    fn entries(&self) -> Vec<Entry<K, V>> {
        todo!()
    }

    fn keys(&self) -> Vec<K> {
        todo!()
    }

    fn values(&self) -> Vec<V> {
        todo!()
    }
}

impl<K: Clone + Hash + Ord, V: Clone> Default for QPHashMap<K, V> {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}
