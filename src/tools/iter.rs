use {
    super::{
        entry::{Entry, EntrySet, IntoEntry, IntoEntrySet, RawEntry},
        misc::base::OrderedMap,
    },
    crate::map::{IntoMapUtil, MapIter, MapUtil},
    std::collections::VecDeque,
};

pub struct Iter<'a, T>(VecDeque<&'a T>);

impl<'a, T> From<Vec<&'a T>> for Iter<'a, T> {
    fn from(entries: Vec<&'a T>) -> Self {
        Self(entries.into())
    }
}

impl<'a, K, V> From<&'a Vec<RawEntry<K, V>>> for Iter<'a, (K, V)> {
    fn from(maps: &'a Vec<RawEntry<K, V>>) -> Self {
        maps.iter().map(EntrySet::set).collect()
    }
}

impl<'a, K, V> From<&'a Vec<OrderedMap<K, V>>> for Iter<'a, (K, V)> {
    fn from(maps: &'a Vec<OrderedMap<K, V>>) -> Self {
        maps.iter().flat_map(OrderedMap::iter).collect()
    }
}

impl<'a, K, V> FromIterator<&'a (K, V)> for Iter<'a, (K, V)> {
    fn from_iter<T: IntoIterator<Item = &'a (K, V)>>(iter: T) -> Self {
        let mut entries = Vec::new();
        iter.into_iter().for_each(|i| entries.push(i));
        Self(entries.into())
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct IntoIter<T>(VecDeque<T>);

impl<T> From<Vec<T>> for IntoIter<T> {
    fn from(entries: Vec<T>) -> Self {
        Self(entries.into())
    }
}

impl<K, V> From<Vec<RawEntry<K, V>>> for IntoIter<(K, V)> {
    fn from(maps: Vec<RawEntry<K, V>>) -> Self {
        maps.into_iter().map(IntoEntrySet::into_set).collect()
    }
}

impl<K, V> From<Vec<OrderedMap<K, V>>> for IntoIter<(K, V)> {
    fn from(maps: Vec<OrderedMap<K, V>>) -> Self {
        maps.into_iter().flat_map(OrderedMap::into_iter).collect()
    }
}

impl<K, V> FromIterator<(K, V)> for IntoIter<(K, V)> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut entries = Vec::new();
        iter.into_iter().for_each(|i| entries.push(i));
        Self(entries.into())
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct Keys<'a, T>(Iter<'a, T>);

impl<'a, K, V> From<&'a Vec<RawEntry<K, V>>> for Keys<'a, K> {
    fn from(entries: &'a Vec<RawEntry<K, V>>) -> Self {
        Self(
            entries
                .into_iter()
                .map(Entry::key)
                .collect::<Vec<&'a K>>()
                .into(),
        )
    }
}

impl<'a, K, V> From<&'a Vec<OrderedMap<K, V>>> for Keys<'a, K> {
    fn from(maps: &'a Vec<OrderedMap<K, V>>) -> Self {
        maps.iter().flat_map(OrderedMap::keys).collect()
    }
}

impl<'a, K> FromIterator<&'a K> for Keys<'a, K> {
    fn from_iter<T: IntoIterator<Item = &'a K>>(iter: T) -> Self {
        let mut keys = Vec::new();
        iter.into_iter().for_each(|i| keys.push(i));
        Self(keys.into())
    }
}

impl<'a, K> Iterator for Keys<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct IntoKeys<T>(IntoIter<T>);

impl<K, V> From<Vec<RawEntry<K, V>>> for IntoKeys<K> {
    fn from(entries: Vec<RawEntry<K, V>>) -> Self {
        Self(
            entries
                .into_iter()
                .map(IntoEntry::into_key)
                .collect::<Vec<K>>()
                .into(),
        )
    }
}

impl<K, V> From<Vec<OrderedMap<K, V>>> for IntoKeys<K> {
    fn from(maps: Vec<OrderedMap<K, V>>) -> Self {
        maps.into_iter().flat_map(OrderedMap::into_keys).collect()
    }
}

impl<'a, K, V> From<&'a Vec<OrderedMap<K, V>>> for Values<'a, V> {
    fn from(maps: &'a Vec<OrderedMap<K, V>>) -> Self {
        maps.iter().flat_map(OrderedMap::values).collect()
    }
}

impl<'a, V> FromIterator<&'a V> for Values<'a, V> {
    fn from_iter<T: IntoIterator<Item = &'a V>>(iter: T) -> Self {
        let mut values = Vec::new();
        iter.into_iter().for_each(|i| values.push(i));
        Self(values.into())
    }
}

impl<K> Iterator for IntoKeys<K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct Values<'a, T>(Iter<'a, T>);

impl<'a, K, V> From<&'a Vec<RawEntry<K, V>>> for Values<'a, V> {
    fn from(entries: &'a Vec<RawEntry<K, V>>) -> Self {
        Self(
            entries
                .into_iter()
                .map(Entry::value)
                .collect::<Vec<&'a V>>()
                .into(),
        )
    }
}

impl<K> FromIterator<K> for IntoKeys<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut keys = Vec::new();
        iter.into_iter().for_each(|i| keys.push(i));
        Self(keys.into())
    }
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct IntoValues<T>(IntoIter<T>);

impl<K, V> From<Vec<RawEntry<K, V>>> for IntoValues<V> {
    fn from(entries: Vec<RawEntry<K, V>>) -> Self {
        Self(
            entries
                .into_iter()
                .map(IntoEntry::into_value)
                .collect::<Vec<V>>()
                .into(),
        )
    }
}

impl<K, V> From<Vec<OrderedMap<K, V>>> for IntoValues<V> {
    fn from(maps: Vec<OrderedMap<K, V>>) -> Self {
        maps.into_iter().flat_map(OrderedMap::into_values).collect()
    }
}

impl<V> FromIterator<V> for IntoValues<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut values = Vec::new();
        iter.into_iter().for_each(|i| values.push(i));
        Self(values.into())
    }
}

impl<V> Iterator for IntoValues<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
