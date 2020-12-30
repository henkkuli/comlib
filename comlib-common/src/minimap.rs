use std::fmt;

/// Minimal map-like data structure. Optimized for small number of elements.
///
/// The API is modelled after [`BTreeMap`] and [`HashMap`] and pruned to contain the absolute minimum.
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`HashMap`]: std::collections::HashMap
#[derive(Clone)]
pub struct MiniMap<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> MiniMap<K, V>
where
    K: Ord,
{
    /// Constructs a new `MiniMap`.
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    /// Inserts the given value at the given key.
    ///
    /// Returns the previous value stored at the key, if one exists.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.data.binary_search_by_key(&&key, |e| &e.0) {
            Ok(idx) => Some(std::mem::replace(&mut self.data[idx], (key, value)).1),
            Err(idx) => {
                self.data.insert(idx, (key, value));
                None
            }
        }
    }

    /// Returns a reference to a value corresponding to the key.
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.data.binary_search_by_key(&key, |e| &e.0) {
            Ok(idx) => Some(&self.data[idx].1),
            Err(_) => None,
        }
    }

    /// Returns a mutable reference to a value corresponding to the key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.data.binary_search_by_key(&key, |e| &e.0) {
            Ok(idx) => Some(&mut self.data[idx].1),
            Err(_) => None,
        }
    }
}

impl<K, V> Default for MiniMap<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> fmt::Debug for MiniMap<K, V>
where
    K: Ord,
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.data.iter().map(|(k, v)| (k, v)))
            .finish()
    }
}
