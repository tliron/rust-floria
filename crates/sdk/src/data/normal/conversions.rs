use super::{
    super::super::dispatch_bindings::{NestedList, NestedMap, Value},
    list::*,
    map::*,
};

use {duplicate::*, std::collections::*};

impl From<List> for Value {
    fn from(list: List) -> Self {
        Self::NestedList(NestedList::new(list))
    }
}

impl From<Map> for Value {
    fn from(map: Map) -> Self {
        Self::NestedMap(NestedMap::new(map))
    }
}

// Conversion from primitives

impl From<()> for Value {
    fn from(_null: ()) -> Self {
        Self::Null
    }
}

#[duplicate_item(
  _From;
  [i64];
  [i32];
  [i16];
  [i8];
  [isize];
)]
impl From<_From> for Value {
    fn from(integer: _From) -> Self {
        Self::Integer(integer as i64)
    }
}

#[duplicate_item(
  _From;
  [u64];
  [u32];
  [u16];
  [u8];
  [usize];
)]
impl From<_From> for Value {
    fn from(unsigned_integer: _From) -> Self {
        Self::UnsignedInteger(unsigned_integer as u64)
    }
}

#[duplicate_item(
  _From;
  [f64];
  [f32];
)]
impl From<_From> for Value {
    fn from(float: _From) -> Self {
        Self::Float(float as f64)
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Self::Text(string)
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Self::Text(string.into())
    }
}

impl From<Vec<u8>> for Value {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Bytes(bytes)
    }
}

impl From<&[u8]> for Value {
    fn from(bytes: &[u8]) -> Self {
        Self::Bytes(bytes.into())
    }
}

impl From<Vec<Value>> for Value {
    fn from(vector: Vec<Value>) -> Self {
        List::new_with(vector).into()
    }
}

impl From<BTreeMap<Value, Value>> for Value {
    fn from(map: BTreeMap<Value, Value>) -> Self {
        Map::new_with(map).into()
    }
}
