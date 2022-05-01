use crate::{Entry, Map, DEFAULT_CAPACITY};

#[derive(Clone, Debug)]
pub struct OrderedMap<K, V>(Vec<Entry<K, V>>);

impl<K: Clone + Ord, V: Clone> Map<K, V> for OrderedMap<K, V> {
    fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn get(&self, key: K) -> Option<V> {
        if let Ok(i) = self.find(key) {
            Some(self.0[i].value())
        } else {
            None
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.find(key.clone()) {
            Ok(i) => {
                let temp = self.0[i].value();
                self.0[i].set_value(value);
                Some(temp)
            }
            Err(i) => {
                self.0.insert(i, Entry::new(key, value));
                None
            }
        }
    }

    fn remove(&mut self, key: K) -> Option<V> {
        Some(self.0.remove(self.find(key).ok()?).value())
    }

    fn entries(&self) -> Vec<Entry<K, V>> {
        self.0.clone()
    }

    fn keys(&self) -> Vec<K> {
        self.0.iter().map(|e| e.key()).collect()
    }

    fn values(&self) -> Vec<V> {
        self.0.iter().map(|e| e.value()).collect()
    }
}

impl<K: Clone + Ord, V: Clone> OrderedMap<K, V> {
    fn find(&self, key: K) -> Result<usize, usize> {
        self.0.binary_search_by_key(&key, |e| e.key())
    }
}

impl<K: Clone + Ord, V: Clone> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}
