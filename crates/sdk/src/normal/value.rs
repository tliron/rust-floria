use super::{super::dispatcher_bindings::Value, list::*, map::*};

impl Value {
    /// Gets a reference to a nested value.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an
    /// [Value::UnsignedInteger] or an [Value::Integer].
    pub fn get(&self, key: &Self) -> Option<&Self> {
        match self {
            Self::NestedMap(nested_map) => {
                let map: &Map = nested_map.get();
                map.value.get(key)
            }

            Self::NestedList(nested_list) => {
                let list: &List = nested_list.get();
                match key {
                    Self::UnsignedInteger(unsigned_integer) => list.value.get(*unsigned_integer as usize),
                    Self::Integer(integer) => list.value.get(*integer as usize),
                    _ => None,
                }
            }

            _ => None,
        }
    }

    /// Gets a reference to a nested value.
    ///
    /// If this is a [Map], the argument is treated as a key.
    ///
    /// If this is a [List], the argument is treated as an index and must be an
    /// [Value::UnsignedInteger] or an [Value::Integer].
    pub fn into_get<KeyT>(&self, key: KeyT) -> Option<&Self>
    where
        KeyT: Into<Self>,
    {
        self.get(&key.into())
    }

    /// Traverse a value by calling [Value::get] repeatedly.
    ///
    /// Any non-collection or missing key will cause the traversal to stop and return [None].
    ///
    /// Use the [traverse!](crate::traverse) macro instead if you can. It will generally
    /// be more efficient because it doesn't require an allocated array.
    pub fn traverse<'own, IterableT>(&self, keys: IterableT) -> Option<&Self>
    where
        IterableT: IntoIterator<Item = &'own Self>,
    {
        let mut found = self;
        for key in keys {
            found = match found.get(key) {
                Some(value) => value,
                None => return None,
            }
        }
        Some(found)
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,

            Self::Integer(integer) => Self::Integer(*integer),

            Self::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),

            Self::Float(float) => Self::Float(*float),

            Self::Boolean(boolean) => Self::Boolean(*boolean),

            Self::Text(text) => Self::Text(text.clone()),

            Self::Bytes(bytes) => Self::Bytes(bytes.clone()),

            Self::NestedList(nested_list) => {
                let list: &List = nested_list.get();
                list.clone().into()
            }

            Self::NestedMap(nested_map) => {
                let map: &Map = nested_map.get();
                map.clone().into()
            }
        }
    }
}
