use super::Entry;

/// A basic interface for a `Map`
///
/// # Example
/// ```
/// use implhm::{Map, SCHashMap};
///
/// fn main() {
///     let mut map = SCHashMap::default();
///
///     map.insert("orange", "ORANGE");
///     map.insert("blueberry", "BLUEBERRY");
///
///     assert_eq!(map.get("orange"), Some("ORANGE"));
///     assert_eq!(map.get("blueberry"), Some("BLUEBERRY"));
/// }
/// ```
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
