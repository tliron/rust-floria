use super::{super::super::dispatch_bindings::*, call::*, list::*, map::*};

use {duplicate::*, std::collections::*};

impl From<List> for Any {
    fn from(list: List) -> Self {
        Self::AnyList(AnyList::new(list))
    }
}

impl From<Map> for Any {
    fn from(map: Map) -> Self {
        Self::AnyMap(AnyMap::new(map))
    }
}

impl From<Call> for Any {
    fn from(call: Call) -> Self {
        Self::AnyCall(AnyCall::new(call))
    }
}

// Conversion from primitives

impl From<()> for Any {
    fn from(_null: ()) -> Self {
        Self::Null
    }
}

#[duplicate_item(
  FromT;
  [i64];
  [i32];
  [i16];
  [i8];
  [isize];
)]
impl From<FromT> for Any {
    fn from(integer: FromT) -> Self {
        Self::Integer(integer as i64)
    }
}

#[duplicate_item(
  FromT;
  [u64];
  [u32];
  [u16];
  [u8];
  [usize];
)]
impl From<FromT> for Any {
    fn from(unsigned_integer: FromT) -> Self {
        Self::UnsignedInteger(unsigned_integer as u64)
    }
}

#[duplicate_item(
  FromT;
  [f64];
  [f32];
)]
impl From<FromT> for Any {
    fn from(float: FromT) -> Self {
        Self::Float(float as f64)
    }
}

impl From<bool> for Any {
    fn from(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<String> for Any {
    fn from(string: String) -> Self {
        Self::Text(string)
    }
}

impl From<&str> for Any {
    fn from(string: &str) -> Self {
        Self::Text(string.into())
    }
}

impl From<Vec<u8>> for Any {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Blob(bytes)
    }
}

impl From<&[u8]> for Any {
    fn from(bytes: &[u8]) -> Self {
        Self::Blob(bytes.into())
    }
}

impl From<Vec<Any>> for Any {
    fn from(vector: Vec<Any>) -> Self {
        List::from(vector).into()
    }
}

impl From<BTreeMap<Any, Any>> for Any {
    fn from(map: BTreeMap<Any, Any>) -> Self {
        Map::from(map).into()
    }
}
