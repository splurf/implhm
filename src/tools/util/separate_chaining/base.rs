use {
    super::{create_table, next_prime, OrderedMap, DEFAULT_CAPACITY},
    crate::{Map, MapMut, MapUtil},
    std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
        mem::replace,
        slice::Iter,
        vec::IntoIter,
    },
};

const MAX_LOAD_FACTOR: f32 = 0.75;

/** HashMap implementation handling *collision* by *separate chaining* */
#[derive(Debug)]
pub struct SCHashMap<K, V>(Vec<OrderedMap<K, V>>);

impl<K, V> SCHashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self(create_table(capacity, OrderedMap::new))
    }

    /** Determine if the current load factor (clf) has reached the maximum by calculating the clf by dividing the current size by the aggregate capacity */
    fn loaded(&self) -> bool {
        (self.len() as f32 / self.0.len().pow(2) as f32) >= MAX_LOAD_FACTOR
    }
}

impl<K: Hash, V> SCHashMap<K, V> {
    /** Calculate a *hash* of the given `key` by using the builtin `DefaultHasher` then modulate the resulting hash by the current capacity to get the *location* of the given `key` */
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.0.len() as u64) as usize
    }
}

impl<K: Hash + Ord, V> SCHashMap<K, V> {
    /** Set the capacity to the next prime integer after double the current capacity then rehash the original data into a new table */
    fn rehash(&mut self) {
        let capacity = next_prime(self.0.capacity() * 2);
        let bucket_capacity = (capacity as f32 * MAX_LOAD_FACTOR) as usize;

        let old = replace(
            &mut self.0,
            create_table(capacity, |_| OrderedMap::new(bucket_capacity)),
        );

        for (k, v) in old.into_iter().flat_map(|o| o.into_iter()) {
            self.insert(k, v);
        }
    }
}

impl<K, V> Default for SCHashMap<K, V> {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}

impl<K: Hash + Ord, V> From<Vec<(K, V)>> for SCHashMap<K, V> {
    fn from(kv: Vec<(K, V)>) -> Self {
        let mut map = Self::default();
        for (key, value) in kv.into_iter() {
            map.insert(key, value);
        }
        map
    }
}

impl<K: Hash + Ord, V> Map<K, V> for SCHashMap<K, V> {
    fn get(&self, key: K) -> Option<&V> {
        Some(self.0[self.hash(&key)].get(key)?)
    }
}

impl<K: Hash + Ord, V> MapMut<K, V> for SCHashMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let i = self.hash(&key);
        let value = self.0[i].insert(key, value);

        if self.loaded() {
            self.rehash()
        }
        value
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let i = self.hash(&key);
        self.0[i].remove(key)
    }
}

impl<K, V> MapUtil<K, V> for SCHashMap<K, V> {
    fn len(&self) -> usize {
        let mut total = 0;
        self.0.iter().for_each(|o| total += o.len());
        total
    }

    fn keys(&self) -> Iter<K> {
        todo!()
    }

    fn values(&self) -> Iter<V> {
        todo!()
    }
}

impl<K, V> IntoIterator for SCHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .flat_map(OrderedMap::into_iter)
            .collect::<Vec<(K, V)>>()
            .into_iter()
    }
}
