use super::Entry;

pub trait Map<K, V>: Default {
    fn new(capacity: usize) -> Self;
    fn len(&self) -> usize;
    fn get(&self, key: K) -> Option<V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: K) -> Option<V>;
    fn entries(&self) -> Vec<Entry<K, V>>;
    fn keys(&self) -> Vec<K>;
    fn values(&self) -> Vec<V>;
}
