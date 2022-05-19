use {
    crate::{
        iter::{IntoIter, IntoKeys, IntoValues, Iter, Keys, Values},
        map::{IntoMapUtil, Map, MapIter, MapMut, MapSize, MapUtil},
        tools::misc::{
            base::OrderedMap,
            constant::{DEFAULT_CAPACITY, MAX_LOAD_FACTOR},
            func::{create_table, next_prime},
        },
    },
    std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
        mem::replace,
    },
};

/** HashMap implementation handling *collision* by *separate chaining* */
#[derive(Debug)]
pub struct SCHashMap<K, V> {
    data: Vec<OrderedMap<K, V>>,
    capacity: usize,
    len: usize,
}

impl<K, V> SCHashMap<K, V> {
    fn new(capacity: usize, len: usize) -> Self {
        Self {
            data: create_table(capacity, || OrderedMap::new(capacity)),
            capacity,
            len,
        }
    }

    /** Determine if the current load factor (clf) has reached the maximum by calculating the clf by dividing the current size by the aggregate capacity */
    fn loaded(&self) -> bool {
        (self.len as f32 / self.capacity as f32) > MAX_LOAD_FACTOR
    }
}

impl<K: Hash, V> SCHashMap<K, V> {
    /** Calculate a *hash* of the given `key` by using the builtin `DefaultHasher` then modulate the resulting hash by the current capacity to get the *location* of the given `key` */
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.capacity as u64) as usize
    }
}

impl<K: Hash + Ord, V> SCHashMap<K, V> {
    /** Set the capacity to the next prime integer after double the current capacity then rehash the original data into a new table */
    fn rehash(&mut self) {
        //  Initialize the new capacity to the next prime number after double the current capacity
        self.capacity = next_prime(self.capacity * 2);

        //  Replace the current map with an empty map with the new capacity
        let old = replace(
            &mut self.data,
            create_table(self.capacity, || OrderedMap::default()),
        );

        //  Perform rehashing
        for (key, value) in IntoIter::from(old) {
            self._insert(key, value);
        }
    }

    fn _insert(&mut self, key: K, value: V) -> Option<V> {
        let i = self.hash(&key);
        self.data[i].insert(key, value)
    }
}

impl<K, V> Default for SCHashMap<K, V> {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY, 0)
    }
}

impl<K: Hash + Ord, V, T: Into<Vec<(K, V)>>> From<T> for SCHashMap<K, V> {
    fn from(v: T) -> Self {
        let mut map = Self::default();
        for (key, value) in v.into().into_iter() {
            map.insert(key, value);
        }
        map
    }
}

impl<K: Hash + Ord, V> Map<K, V> for SCHashMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        Some(self.data[self.hash(key)].get(key)?)
    }
}

impl<K: Hash + Ord, V> MapMut<K, V> for SCHashMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let value = self._insert(key, value);

        if value.is_none() {
            self.len += 1
        }

        if self.loaded() {
            self.rehash()
        }
        value
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let i = self.hash(key);
        let value = self.data[i].remove(key);
        if value.is_some() {
            self.len -= 1
        }
        value
    }
}

impl<'a, K, V> MapUtil<'a, K, V> for SCHashMap<K, V> {
    fn keys(&'a self) -> Keys<'a, K> {
        (&self.data).into()
    }

    fn values(&'a self) -> Values<'a, V> {
        (&self.data).into()
    }
}

impl<K, V> IntoMapUtil<K, V> for SCHashMap<K, V> {
    fn into_keys(self) -> IntoKeys<K> {
        self.data.into()
    }

    fn into_values(self) -> IntoValues<V> {
        self.data.into()
    }
}

impl<'a, K, V> MapIter<'a, K, V> for SCHashMap<K, V> {
    fn iter(&'a self) -> Iter<'a, (K, V)> {
        (&self.data).into()
    }
}

impl<K, V> MapSize for SCHashMap<K, V> {
    fn len(&self) -> usize {
        self.len
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<K, V> IntoIterator for SCHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into()
    }
}
