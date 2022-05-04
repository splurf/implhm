use std::fmt::Debug;

pub trait Entry<K, V> {
    fn key(&self) -> &K;
    fn value(&self) -> &V;
    fn into_key(self) -> K;
    fn into_value(self) -> V;
}

pub trait EntryMut<K, V>: Entry<K, V> {
    fn key_mut(&mut self) -> &mut K;
    fn value_mut(&mut self) -> &mut V;
}

pub trait EntrySet<K, V>: Entry<K, V> {
    fn into_set(self) -> (K, V);
}

/** Basic `Entry` for each `HashMap` implementation */
#[derive(Debug, Default)]
pub struct RawEntry<K, V> {
    key: K,
    value: V,
}

impl<K, V> RawEntry<K, V> {
    /** Create a new `Entry` with the specified `key` and `value` */
    pub fn new(key: K, value: V) -> Self {
        RawEntry { key, value }
    }
}

impl<K, V> Entry<K, V> for RawEntry<K, V> {
    /** Return a reference of the key */
    fn key(&self) -> &K {
        &self.key
    }

    /** Return a reference of the value */
    fn value(&self) -> &V {
        &self.value
    }

    /** Return the original key, consuming the entry */
    fn into_key(self) -> K {
        self.key
    }

    /** Return the original value, consuming the entry */
    fn into_value(self) -> V {
        self.value
    }
}

impl<K, V> EntryMut<K, V> for RawEntry<K, V> {
    /** Return a mutable reference of the key */
    fn key_mut(&mut self) -> &mut K {
        &mut self.key
    }

    /** Return a mutable reference of the value */
    fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
}

impl<K, V> EntrySet<K, V> for RawEntry<K, V> {
    fn into_set(self) -> (K, V) {
        (self.key, self.value)
    }
}
