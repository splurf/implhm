use {
    crate::{
        entry::RawEntry,
        iter::{IntoIter, IntoKeys, IntoValues, Iter, Keys, Values},
        map::{IntoMapUtil, Map, MapIter, MapMut, MapSize, MapUtil},
        tools::misc::{constant::DEFAULT_CAPACITY, func::create_table},
    },
    std::hash::Hash,
};

/** HashMap implementation handling *collision* by *double hashing* */
#[derive(Debug)]
pub struct LPHashMap<K, V>(Vec<Option<RawEntry<K, V>>>);

impl<K, V> LPHashMap<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self(create_table(capacity, || None))
    }
}

impl<K, V> Map<K, V> for LPHashMap<K, V> {
    fn get(&self, _key: &K) -> Option<&V> {
        todo!()
    }
}

impl<K, V> MapMut<K, V> for LPHashMap<K, V> {
    fn insert(&mut self, _key: K, _value: V) -> Option<V> {
        todo!()
    }

    fn remove(&mut self, _key: &K) -> Option<V> {
        todo!()
    }
}

impl<'a, K, V> MapUtil<'a, K, V> for LPHashMap<K, V> {
    fn keys(&'a self) -> Keys<'a, K> {
        todo!()
    }

    fn values(&'a self) -> Values<'a, V> {
        todo!()
    }
}

impl<K, V> IntoMapUtil<K, V> for LPHashMap<K, V> {
    fn into_keys(self) -> IntoKeys<K> {
        todo!()
    }

    fn into_values(self) -> IntoValues<V> {
        todo!()
    }
}

impl<K, V> IntoIterator for LPHashMap<K, V> {
    type Item = (K, V);

    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<'a, K, V> MapIter<'a, K, V> for LPHashMap<K, V> {
    fn iter(&'a self) -> Iter<'a, (K, V)> {
        todo!()
    }
}

impl<K, V> MapSize for LPHashMap<K, V> {
    fn len(&self) -> usize {
        todo!()
    }
}

impl<K, V> Default for LPHashMap<K, V> {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}

impl<K: Hash + Ord, V> From<Vec<(K, V)>> for LPHashMap<K, V> {
    fn from(kv: Vec<(K, V)>) -> Self {
        let mut map = Self::default();
        for (key, value) in kv.into_iter() {
            map.insert(key, value);
        }
        map
    }
}
