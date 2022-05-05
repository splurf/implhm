use {
    super::entry::{Entry, EntrySet, IntoEntry, RawEntry},
    std::collections::VecDeque,
};

pub struct Iter<'a, T>(VecDeque<&'a T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<'a, T> From<Vec<&'a T>> for Iter<'a, T> {
    fn from(entries: Vec<&'a T>) -> Self {
        Self(entries.into())
    }
}

impl<'a, K, V> From<&'a Vec<RawEntry<K, V>>> for Iter<'a, (K, V)> {
    fn from(maps: &'a Vec<RawEntry<K, V>>) -> Self {
        maps.iter().map(|e| e.set()).collect()
    }
}

impl<'a, K, V> FromIterator<&'a (K, V)> for Iter<'a, (K, V)> {
    fn from_iter<T: IntoIterator<Item = &'a (K, V)>>(iter: T) -> Self {
        let mut entries = Vec::new();
        iter.into_iter().for_each(|i| entries.push(i));
        Self(entries.into())
    }
}

impl<'a, K, V> FromIterator<Iter<'a, (K, V)>> for Iter<'a, (K, V)> {
    fn from_iter<T: IntoIterator<Item = Iter<'a, (K, V)>>>(iter: T) -> Self {
        let mut entries = Vec::new();
        iter.into_iter()
            .for_each(|i| i.for_each(|j| entries.push(j)));
        Self(entries.into())
    }
}

pub struct IntoIter<T>(VecDeque<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> From<Vec<T>> for IntoIter<T> {
    fn from(entries: Vec<T>) -> Self {
        Self(entries.into())
    }
}

pub struct Keys<'a, T>(Iter<'a, T>);

impl<'a, K, V> From<&'a Vec<RawEntry<K, V>>> for Keys<'a, K> {
    fn from(entries: &'a Vec<RawEntry<K, V>>) -> Self {
        Self(
            entries
                .into_iter()
                .map(|e| e.key())
                .collect::<Vec<&'a K>>()
                .into(),
        )
    }
}

impl<'a, K> Iterator for Keys<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, K> FromIterator<Keys<'a, K>> for Keys<'a, K> {
    fn from_iter<T: IntoIterator<Item = Keys<'a, K>>>(iter: T) -> Self {
        let mut keys = Vec::new();
        iter.into_iter().for_each(|i| i.for_each(|j| keys.push(j)));
        Self(keys.into())
    }
}

pub struct IntoKeys<T>(IntoIter<T>);

impl<K, V> From<Vec<RawEntry<K, V>>> for IntoKeys<K> {
    fn from(entries: Vec<RawEntry<K, V>>) -> Self {
        Self(
            entries
                .into_iter()
                .map(|e| e.into_key())
                .collect::<Vec<K>>()
                .into(),
        )
    }
}

impl<K> Iterator for IntoKeys<K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, V> FromIterator<Values<'a, V>> for Values<'a, V> {
    fn from_iter<T: IntoIterator<Item = Values<'a, V>>>(iter: T) -> Self {
        let mut values = Vec::new();
        iter.into_iter()
            .for_each(|i| i.for_each(|j| values.push(j)));
        Self(values.into())
    }
}

pub struct Values<'a, T>(Iter<'a, T>);

impl<'a, K, V> From<&'a Vec<RawEntry<K, V>>> for Values<'a, V> {
    fn from(entries: &'a Vec<RawEntry<K, V>>) -> Self {
        Self(
            entries
                .into_iter()
                .map(|e| e.value())
                .collect::<Vec<&'a V>>()
                .into(),
        )
    }
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<K> FromIterator<IntoKeys<K>> for IntoKeys<K> {
    fn from_iter<T: IntoIterator<Item = IntoKeys<K>>>(iter: T) -> Self {
        let mut keys = Vec::new();
        iter.into_iter().for_each(|i| i.for_each(|j| keys.push(j)));
        Self(keys.into())
    }
}

pub struct IntoValues<T>(IntoIter<T>);

impl<K, V> From<Vec<RawEntry<K, V>>> for IntoValues<V> {
    fn from(entries: Vec<RawEntry<K, V>>) -> Self {
        Self(
            entries
                .into_iter()
                .map(|e| e.into_value())
                .collect::<Vec<V>>()
                .into(),
        )
    }
}

impl<V> Iterator for IntoValues<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<V> FromIterator<IntoValues<V>> for IntoValues<V> {
    fn from_iter<T: IntoIterator<Item = IntoValues<V>>>(iter: T) -> Self {
        let mut values = Vec::new();
        iter.into_iter()
            .for_each(|i| i.for_each(|j| values.push(j)));
        Self(values.into())
    }
}
