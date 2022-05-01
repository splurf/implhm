use {
    super::OrderedMap,
    crate::{Entry, Map, DEFAULT_CAPACITY},
    std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    },
};

const MAX_LOAD_FACTOR: f32 = 0.75;

#[derive(Debug)]
pub struct SCHashMap<K, V> {
    data: Vec<OrderedMap<K, V>>,
    capacity: usize,
}

impl<K: Clone + Hash + Ord, V: Clone> Map<K, V> for SCHashMap<K, V> {
    fn new(capacity: usize) -> Self {
        Self {
            data: Self::create_table(capacity),
            capacity,
        }
    }

    fn len(&self) -> usize {
        let mut total = 0;
        self.data.iter().for_each(|o| total += o.len());
        total
    }

    fn get(&self, key: K) -> Option<V> {
        Some(self.data[self.hash(key.clone())].get(key)?)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let i = self.hash(key.clone());
        let value = self.data[i].insert(key, value);

        if self.loaded() {
            self.grow()
        }
        value
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let i = self.hash(key.clone());
        self.data[i].remove(key)
    }

    fn entries(&self) -> Vec<Entry<K, V>> {
        self.data.iter().map(|o| o.entries()).flatten().collect()
    }

    fn keys(&self) -> Vec<K> {
        self.data.iter().map(|o| o.keys()).flatten().collect()
    }

    fn values(&self) -> Vec<V> {
        self.data.iter().map(|o| o.values()).flatten().collect()
    }
}

impl<K: Clone + Hash + Ord, V: Clone> SCHashMap<K, V> {
    fn create_table(capacity: usize) -> Vec<OrderedMap<K, V>> {
        (0..capacity).map(|_| OrderedMap::new(capacity)).collect()
    }

    fn loaded(&self) -> bool {
        (self.len() as f32 / self.capacity.pow(2) as f32) >= MAX_LOAD_FACTOR
    }

    fn grow(&mut self) {
        self.capacity *= 2;

        let buffer = self.entries();

        self.data = Self::create_table(self.capacity);

        buffer.into_iter().for_each(|e| {
            self.insert(e.key(), e.value());
        })
    }

    fn hash(&self, key: K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.capacity as u64) as usize
    }
}

impl<K: Clone + Hash + Ord, V: Clone> From<Vec<(K, V)>> for SCHashMap<K, V> {
    fn from(kv: Vec<(K, V)>) -> Self {
        let mut map = Self::new(DEFAULT_CAPACITY);
        for (key, value) in kv.into_iter() {
            map.insert(key, value);
        }
        map
    }
}

impl<K: Clone + Hash + Ord, V: Clone> Default for SCHashMap<K, V> {
    fn default() -> Self {
        let capacity = DEFAULT_CAPACITY;
        Self {
            data: Self::create_table(capacity),
            capacity,
        }
    }
}
