use std::fmt::Debug;

/** Basic `Entry` for each `HashMap` implementation */
#[derive(Clone, Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K: Clone, V: Clone> Entry<K, V> {
    /** Create a new `Entry` with the specified `key` and `value` */
    pub fn new(key: K, value: V) -> Self {
        Entry { key, value }
    }

    /** Return a copy of the key */
    pub fn key(&self) -> K {
        self.key.clone()
    }

    /** Return a copy of the value */
    pub fn value(&self) -> V {
        self.value.clone()
    }

    /** Replace the value of the `Entry` with `value` */
    pub fn set_value(&mut self, value: V) {
        self.value = value;
    }
}
