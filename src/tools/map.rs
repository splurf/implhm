use {
    super::iter::{Iter, Keys, Values},
    crate::iter::{IntoKeys, IntoValues},
};

/** A basic interface for a `Map` */
pub trait Map<K, V> {
    fn get(&self, key: K) -> Option<&V>;
}

pub trait MapMut<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: K) -> Option<V>;
}

pub trait MapUtil<'a, K, V> {
    fn keys(&'a self) -> Keys<'a, K>;
    fn values(&'a self) -> Values<'a, V>;
}

pub trait IntoMapUtil<K, V> {
    fn into_keys(self) -> IntoKeys<K>;
    fn into_values(self) -> IntoValues<V>;
}

pub trait MapIter<'a, K, V>: IntoIterator {
    fn iter(&'a self) -> Iter<'a, (K, V)>;
}

pub trait MapSize {
    fn len(&self) -> usize;
}
