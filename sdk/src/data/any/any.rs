use super::super::super::dispatch_bindings::*;

use std::mem::*;

impl Any {
    /// Gets a reference to an inner [Any].
    ///
    /// If this is a map, the argument is treated as a key.
    ///
    /// If this is a list, the argument is treated as an index and must be an
    /// [Any::UnsignedInteger] or an [Any::Integer].
    pub fn get(&self, key: &Self) -> Option<&Self> {
        match self {
            Self::AnyMap(any_map) => any_map.to_map().inner.get(key),

            Self::AnyList(any_list) => {
                let index = match key {
                    Self::UnsignedInteger(unsigned_integer) => *unsigned_integer as usize,
                    Self::Integer(integer) => *integer as usize,
                    _ => return None,
                };

                any_list.to_list().inner.get(index)
            }

            _ => None,
        }
    }

    /// Gets a reference to an inner [Any].
    ///
    /// If this is a map, the argument is treated as a key.
    ///
    /// If this is a list, the argument is treated as an index and must be an
    /// [Any::UnsignedInteger] or an [Any::Integer].
    pub fn into_get<KeyT>(&self, key: KeyT) -> Option<&Self>
    where
        KeyT: Into<Self>,
    {
        self.get(&key.into())
    }

    /// Traverse this [Any] by calling [Any::get] repeatedly.
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
            found = found.get(key)?;
        }
        Some(found)
    }

    /// Compare type.
    pub fn same_type(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }

    /// Type name.
    pub fn type_name(&self) -> &str {
        match self {
            Self::Null => "null",
            Self::Integer(_) => "integer",
            Self::UnsignedInteger(_) => "unsigned integer",
            Self::Float(_) => "float",
            Self::Boolean(_) => "boolean",
            Self::Text(_) => "text",
            Self::Blob(_) => "blob",
            Self::AnyList(_) => "list",
            Self::AnyMap(_) => "map",
            Self::AnyCall(_) => "call",
        }
    }
}

impl Clone for Any {
    fn clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,
            Self::Integer(integer) => Self::Integer(*integer),
            Self::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),
            Self::Float(float) => Self::Float(*float),
            Self::Boolean(boolean) => Self::Boolean(*boolean),
            Self::Text(text) => Self::Text(text.clone()),
            Self::Blob(blob) => Self::Blob(blob.clone()),
            Self::AnyList(any_list) => any_list.to_list().clone().into(),
            Self::AnyMap(any_map) => any_map.to_map().clone().into(),
            Self::AnyCall(any_call) => any_call.to_call().clone().into(),
        }
    }
}
