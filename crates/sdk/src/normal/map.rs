use super::{super::dispatcher_bindings::GuestNestedMap, NestedMap, Value};

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
#[derive(Default, Clone)]
pub struct Map {
    /// Actual value.
    pub value: BTreeMap<Value, Value>,
}

impl Map {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructor.
    pub fn new_with<MapT>(value: MapT) -> Self
    where
        MapT: Into<BTreeMap<Value, Value>>,
    {
        Self { value: value.into() }
    }

    /// Constructor.
    pub fn new_from<IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<Item = (Value, Value)>,
    {
        Self::new_with(BTreeMap::from_iter(iterable))
    }
}

impl GuestNestedMap for Map {
    fn new(key_value_pairs: Vec<(Value, Value)>) -> Self {
        Self::new_from(key_value_pairs)
    }

    fn get(&self) -> Vec<(Value, Value)> {
        self.value.clone().into_iter().collect()
    }

    fn length(&self) -> u64 {
        self.value.len() as u64
    }
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char('{')?;

        let mut iterator = self.value.iter().peekable();
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
        self.value.hash(state);
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Map {}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
