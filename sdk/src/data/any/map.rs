use super::super::super::dispatch_bindings::*;

use std::{
    cmp::*,
    collections::*,
    fmt::{self, Write},
    hash::*,
};

//
// NestedMap
//

impl NestedMap {
    /// To map.
    pub fn to_map(&self) -> &Map {
        self.get()
    }

    /// To map.
    pub fn to_map_mut(&mut self) -> &mut Map {
        self.get_mut()
    }
}

//
// Map
//

/// Map.
#[derive(Clone, Default)]
pub struct Map {
    /// Inner.
    pub inner: BTreeMap<Any, Any>,
}

impl Map {
    /// Get.
    pub fn into_get<KeyT>(&self, key: KeyT) -> Option<&Any>
    where
        KeyT: Into<Any>,
    {
        self.inner.get(&key.into())
    }
}

impl From<BTreeMap<Any, Any>> for Map {
    fn from(inner: BTreeMap<Any, Any>) -> Self {
        Self { inner }
    }
}

impl GuestNestedMap for Map {
    fn new(key_value_pairs: Vec<(Any, Any)>) -> Self {
        Self::from_iter(key_value_pairs)
    }

    fn get(&self) -> Vec<(Any, Any)> {
        self.inner.clone().into_iter().collect()
    }

    fn length(&self) -> u64 {
        self.inner.len() as u64
    }
}

impl FromIterator<(Any, Any)> for Map {
    fn from_iter<IntoIteratorT>(iter: IntoIteratorT) -> Self
    where
        IntoIteratorT: IntoIterator<Item = (Any, Any)>,
    {
        Self::from(BTreeMap::from_iter(iter))
    }
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char('{')?;

        let mut iterator = self.inner.iter().peekable();
        while let Some((key, value)) = iterator.next() {
            fmt::Display::fmt(key, formatter)?;
            formatter.write_char(':')?;
            fmt::Display::fmt(value, formatter)?;
            if iterator.peek().is_some() {
                formatter.write_char(',')?;
            }
        }

        formatter.write_char('}')
    }
}

// Delegated

impl Hash for Map {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.inner.hash(state);
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for Map {}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}
