use crate::map::MapSize;

use {
    crate::{
        entry::{Entry, EntryMut, IntoEntry, IntoEntrySet, RawEntry},
        iter::{IntoKeys, IntoValues, Iter, Keys, Values},
        map::{IntoMapUtil, Map, MapIter, MapMut, MapUtil},
    },
    std::{mem::replace, vec::IntoIter},
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
    fn get(&self, key: K) -> Option<&V> {
        if let Ok(i) = self.locate(&key) {
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

    fn remove(&mut self, key: K) -> Option<V> {
        let i = self.locate(&key).ok()?;
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
}

impl<K, V> IntoIterator for OrderedMap<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(IntoEntrySet::into_set)
            .collect::<Vec<(K, V)>>()
            .into_iter()
    }
}

impl<'a, K, V> From<&'a Vec<OrderedMap<K, V>>> for Keys<'a, K> {
    fn from(maps: &'a Vec<OrderedMap<K, V>>) -> Self {
        maps.iter().map(|o| o.keys()).collect()
    }
}

impl<'a, K, V> From<&'a Vec<OrderedMap<K, V>>> for Values<'a, V> {
    fn from(maps: &'a Vec<OrderedMap<K, V>>) -> Self {
        maps.iter().map(|o| o.values()).collect()
    }
}

impl<K, V> From<Vec<OrderedMap<K, V>>> for IntoKeys<K> {
    fn from(maps: Vec<OrderedMap<K, V>>) -> Self {
        maps.into_iter().map(|o| o.into_keys()).collect()
    }
}

impl<K, V> From<Vec<OrderedMap<K, V>>> for IntoValues<V> {
    fn from(maps: Vec<OrderedMap<K, V>>) -> Self {
        maps.into_iter().map(|o| o.into_values()).collect()
    }
}

impl<'a, K, V> From<&'a Vec<OrderedMap<K, V>>> for Iter<'a, (K, V)> {
    fn from(maps: &'a Vec<OrderedMap<K, V>>) -> Self {
        maps.iter().map(|o| o.iter()).collect()
    }
}
