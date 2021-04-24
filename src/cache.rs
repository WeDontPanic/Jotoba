use std::collections::HashMap;
use std::hash::Hash;

// Implement our own cache type
pub struct SharedCache<K: Hash + Eq, V: Clone> {
    store: HashMap<K, V>,
    capacity: usize,
}

impl<K: Hash + Eq, V: Clone> SharedCache<K, V> {
    pub fn with_capacity(size: usize) -> SharedCache<K, V> {
        SharedCache {
            store: HashMap::with_capacity(size),
            capacity: size,
        }
    }
}

impl<K: Hash + Eq, V: Clone> SharedCache<K, V> {
    pub fn cache_get(&self, k: &K) -> Option<&V> {
        self.store.get(k)
    }

    pub fn cache_get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.store.get_mut(k)
    }

    pub fn cache_get_or_set_with<F: FnOnce() -> V>(&mut self, k: K, f: F) -> &mut V {
        self.store.entry(k).or_insert_with(f)
    }

    pub fn cache_set(&mut self, k: K, v: V) -> Option<V> {
        self.store.insert(k, v)
    }

    pub fn cache_remove(&mut self, k: &K) -> Option<V> {
        self.store.remove(k)
    }

    pub fn cache_clear(&mut self) {
        self.store.clear();
    }

    pub fn cache_reset(&mut self) {
        self.store = HashMap::with_capacity(self.capacity);
    }

    pub fn cache_size(&self) -> usize {
        self.store.len()
    }

    pub fn find_by_predicate<'a, P>(&'a self, mut predicate: P) -> Option<&'a V>
    where
        P: FnMut(&&V) -> bool,
    {
        self.store.values().find(|i| predicate(i))
    }

    /// Load multiple values. Skip non cached ones
    pub fn get_values(&mut self, keys: &[K]) -> Vec<V> {
        keys.iter()
            .filter_map(|i| self.cache_get(i).map(|i| i.to_owned()))
            .collect()
    }

    pub fn filter_values<'a, P>(&'a self, mut predicate: P) -> Vec<V>
    where
        P: FnMut(&&V) -> bool,
    {
        self.store
            .values()
            .filter(|i| predicate(i))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn extend<C>(&mut self, items: Vec<V>, mut convert: C)
    where
        C: FnMut(&V) -> K,
    {
        for i in items {
            self.store.entry(convert(&i)).or_insert(i);
        }
    }
}
