use std::fmt::Debug;

pub trait Entry<K, V> {
    fn key(&self) -> &K;
    fn value(&self) -> &V;
}

pub trait EntryMut<K, V> {
    fn key_mut(&mut self) -> &mut K;
    fn value_mut(&mut self) -> &mut V;
}

pub trait IntoEntry<K, V> {
    fn into_key(self) -> K;
    fn into_value(self) -> V;
}

pub trait EntrySet<'a, K, V>: Entry<K, V> {
    fn set(&'a self) -> &'a (K, V);
}

pub trait IntoEntrySet<K, V>: IntoEntry<K, V> {
    fn into_set(self) -> (K, V);
}

/** Basic `Entry` for each `HashMap` implementation */
#[derive(Debug, Default)]
pub struct RawEntry<K, V>((K, V));

impl<K, V> RawEntry<K, V> {
    /** Create a new `Entry` with the specified `key` and `value` */
    pub fn new(key: K, value: V) -> Self {
        RawEntry((key, value))
    }
}

impl<K, V> Entry<K, V> for RawEntry<K, V> {
    /** Return a reference of the key */
    fn key(&self) -> &K {
        &self.0 .0
    }

    /** Return a reference of the value */
    fn value(&self) -> &V {
        &self.0 .1
    }
}

impl<K, V> EntryMut<K, V> for RawEntry<K, V> {
    /** Return a mutable reference of the key */
    fn key_mut(&mut self) -> &mut K {
        &mut self.0 .0
    }

    /** Return a mutable reference of the value */
    fn value_mut(&mut self) -> &mut V {
        &mut self.0 .1
    }
}

impl<'a, K, V> EntrySet<'a, K, V> for RawEntry<K, V> {
    fn set(&'a self) -> &'a (K, V) {
        &self.0
    }
}

impl<K, V> IntoEntry<K, V> for RawEntry<K, V> {
    /** Return the original key, consuming the entry */
    fn into_key(self) -> K {
        self.0 .0
    }

    /** Return the original value, consuming the entry */
    fn into_value(self) -> V {
        self.0 .1
    }
}

impl<K, V> IntoEntrySet<K, V> for RawEntry<K, V> {
    fn into_set(self) -> (K, V) {
        (self.0 .0, self.0 .1)
    }
}
