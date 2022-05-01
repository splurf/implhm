use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K: Clone, V: Clone> Entry<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Entry { key, value }
    }

    pub fn key(&self) -> K {
        self.key.clone()
    }

    pub fn value(&self) -> V {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: V) {
        self.value = value;
    }
}
