use std::fmt;

/// Map data structure optimized for small number of elements.
///
/// The API is modelled after [`BTreeMap`] and [`HashMap`]. Implements [Entry API] for easier access and modification of
/// the map, modelled after [Standard library Entry API].
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`HashMap`]: std::collections::HashMap
/// [Entry API]:
/// [Standard library Entry API]: https://doc.rust-lang.org/std/collections/index.html#entries
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

    /// Turns the map into an ordered vector of key-value pairs.
    pub fn into_inner(self) -> Vec<(K, V)> {
        self.data
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.data.binary_search_by_key(&&key, |e| &e.0) {
            Ok(idx) => Entry::Occupied(OccupiedEntry {
                data: &mut self.data,
                index: idx,
            }),
            Err(idx) => Entry::Vacant(VacantEntry {
                data: &mut self.data,
                key,
                insert_position: idx,
            }),
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

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` inst constructed from the [`entry`] method on [`MiniMap`].
///
/// [`entry`]: MiniMap::entry
pub enum Entry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V>),
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    /// Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value
    /// in the entry.
    pub fn or_insert(self, default: V) -> &'a mut V {
        self.or_insert_with(|| default)
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty, and returns a mutable
    /// reference to the value in the entry.
    pub fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Self::Vacant(entry) => entry.insert(default()),
            Self::Occupied(entry) => entry.into_mut(),
        }
    }

    /// Returns a reference to this entry's key.
    pub fn key(&self) -> &K {
        match self {
            Self::Vacant(entry) => entry.key(),
            Self::Occupied(entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any potential inserts into the map.
    pub fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Self::Vacant(_) => self,
            Self::Occupied(ref mut entry) => {
                f(entry.get_mut());
                self
            }
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value
    /// in the entry.
    pub fn or_default<F>(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert_with(Default::default)
    }
}

/// A view into a vacant entry in a [`MiniMap`]. It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    data: &'a mut Vec<(K, V)>,
    key: K,
    insert_position: usize,
}

impl<'a, K, V> VacantEntry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    /// Gets a reference to the key that would be used when inserting a value through the `VacantEntry`.
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Take ownership of the key.
    pub fn into_key(self) -> K {
        self.key
    }

    /// Sets the value of the entry with the `VacantEntry`'s ye, and returns a mutable reference to it.
    pub fn insert(self, value: V) -> &'a mut V {
        self.data.insert(self.insert_position, (self.key, value));
        &mut self.data[self.insert_position].1
    }
}

/// A view into an occupied entry in a [`MiniMap`]. It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    data: &'a mut Vec<(K, V)>,
    index: usize,
}

impl<'a, K, V> OccupiedEntry<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    /// Gets a reference to the key in the entry.
    pub fn key(&self) -> &K {
        &self.data[self.index].0
    }

    /// Takes the ownership of the key and value from the map.
    pub fn remove_entry(self) -> (K, V) {
        self.data.remove(self.index)
    }

    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &V {
        &self.data[self.index].1
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.data[self.index].1
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry with a lifetime bound to the map
    /// itself.
    pub fn into_mut(self) -> &'a mut V {
        &mut self.data[self.index].1
    }

    /// Sets the value of the entry, and returns the entry's old value.
    pub fn insert(&mut self, value: V) -> V {
        std::mem::replace(&mut self.data[self.index].1, value)
    }

    /// Takes the value out of the entry, and returns it.
    pub fn remove(self) -> V {
        self.remove_entry().1
    }
}
