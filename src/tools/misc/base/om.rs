use {
    crate::{
        entry::{Entry, EntryMut, IntoEntry, RawEntry},
        iter::{IntoIter, IntoKeys, IntoValues, Iter, Keys, Values},
        map::{IntoMapUtil, Map, MapIter, MapMut, MapSize, MapUtil},
        tools::misc::constant::DEFAULT_CAPACITY,
    },
    std::mem::replace,
};

#[derive(Debug)]
pub struct OrderedMap<K, V>(Vec<RawEntry<K, V>>);

impl<K, V> OrderedMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<K: Ord, V> OrderedMap<K, V> {
    fn locate(&self, key: &K) -> Result<usize, usize> {
        self.0.binary_search_by_key(&key, |f| f.key())
    }
}

impl<K: Ord, V> Map<K, V> for OrderedMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        if let Ok(i) = self.locate(key) {
            Some(self.0[i].value())
        } else {
            None
        }
    }
}

impl<K: Ord, V> MapMut<K, V> for OrderedMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.locate(&key) {
            Ok(i) => Some(replace(self.0[i].value_mut(), value)),
            Err(i) => {
                self.0.insert(i, RawEntry::new(key, value));
                None
            }
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let i = self.locate(key).ok()?;
        Some(self.0.remove(i).into_value())
    }
}

impl<'a, K, V> MapUtil<'a, K, V> for OrderedMap<K, V> {
    fn keys(&'a self) -> Keys<'a, K> {
        (&self.0).into()
    }

    fn values(&'a self) -> Values<'a, V> {
        (&self.0).into()
    }
}

impl<K, V> IntoMapUtil<K, V> for OrderedMap<K, V> {
    fn into_keys(self) -> IntoKeys<K> {
        self.0.into()
    }

    fn into_values(self) -> IntoValues<V> {
        self.0.into()
    }
}

impl<'a, K, V> MapIter<'a, K, V> for OrderedMap<K, V> {
    fn iter(&'a self) -> Iter<'a, (K, V)> {
        (&self.0).into()
    }
}

impl<K, V> MapSize for OrderedMap<K, V> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

impl<K, V> IntoIterator for OrderedMap<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into()
    }
}

impl<K, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}
