use std::slice::Iter;

/** A basic interface for a `Map` */
pub trait Map<K, V> {
    fn get(&self, key: K) -> Option<&V>;
}

pub trait MapMut<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: K) -> Option<V>;
}

pub trait MapUtil<K, V>: IntoIterator {
    fn len(&self) -> usize;
    fn keys(&self) -> Iter<K>;
    fn values(&self) -> Iter<V>;
}
