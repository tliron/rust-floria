use super::super::dispatcher_bindings::Value;

use {
    ordered_float::*,
    std::{cmp::*, fmt, hash::*},
};

impl Hash for Value {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Null => ().hash(state),

            Self::Integer(integer) => integer.hash(state),

            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.hash(state),

            Self::Float(float) => OrderedFloat::from(*float).hash(state),

            Self::Boolean(boolean) => boolean.hash(state),

            Self::Text(text) => text.hash(state),

            Self::Bytes(bytes) => bytes.hash(state),

            Self::NestedList(nested_list) => nested_list.to_list().hash(state),

            Self::NestedMap(nested_map) => nested_map.to_map().hash(state),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Null => matches!(other, Self::Null),

            Self::Integer(integer) => {
                if let Self::Integer(other_integer) = other {
                    integer == other_integer
                } else {
                    false
                }
            }

            Self::UnsignedInteger(unsigned_integer) => {
                if let Self::UnsignedInteger(other_unsigned_integer) = other {
                    unsigned_integer == other_unsigned_integer
                } else {
                    false
                }
            }

            Self::Float(float) => {
                if let Self::Float(other_float) = other {
                    float == other_float
                } else {
                    false
                }
            }

            Self::Boolean(boolean) => {
                if let Self::Boolean(other_boolean) = other {
                    boolean == other_boolean
                } else {
                    false
                }
            }

            Self::Text(text) => {
                if let Self::Text(other_text) = other {
                    text == other_text
                } else {
                    false
                }
            }

            Self::Bytes(bytes) => {
                if let Self::Bytes(other_bytes) = other {
                    bytes == other_bytes
                } else {
                    false
                }
            }

            Self::NestedList(nested_list) => {
                if let Self::NestedList(other_nested_list) = other {
                    nested_list.to_list() == other_nested_list.to_list()
                } else {
                    false
                }
            }

            Self::NestedMap(nested_map) => {
                if let Self::NestedMap(other_nested_map) = other {
                    nested_map.to_map() == other_nested_map.to_map()
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Null => {
                if matches!(other, Self::Null) {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            Self::Integer(integer) => {
                if let Self::Integer(other_integer) = other {
                    integer.partial_cmp(other_integer)
                } else {
                    None
                }
            }

            Self::UnsignedInteger(unsigned_integer) => {
                if let Self::UnsignedInteger(other_unsigned_integer) = other {
                    unsigned_integer.partial_cmp(other_unsigned_integer)
                } else {
                    None
                }
            }

            Self::Float(float) => {
                if let Self::Float(other_float) = other {
                    float.partial_cmp(other_float)
                } else {
                    None
                }
            }

            Self::Boolean(boolean) => {
                if let Self::Boolean(other_boolean) = other {
                    boolean.partial_cmp(other_boolean)
                } else {
                    None
                }
            }

            Self::Text(text) => {
                if let Self::Text(other_text) = other {
                    text.partial_cmp(other_text)
                } else {
                    None
                }
            }

            Self::Bytes(bytes) => {
                if let Self::Bytes(other_bytes) = other {
                    bytes.partial_cmp(other_bytes)
                } else {
                    None
                }
            }

            Self::NestedList(nested_list) => {
                if let Self::NestedList(other_nested_list) = other {
                    nested_list.to_list().partial_cmp(other_nested_list.to_list())
                } else {
                    None
                }
            }

            Self::NestedMap(nested_map) => {
                if let Self::NestedMap(other_nested_map) = other {
                    nested_map.to_map().partial_cmp(other_nested_map.to_map())
                } else {
                    None
                }
            }
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Null => match other {
                Self::Null => Ordering::Equal,
                _ => Ordering::Less,
            },

            Value::Integer(integer) => match other {
                Self::Null => Ordering::Greater,
                Self::Integer(other_integer) => integer.cmp(other_integer),
                _ => Ordering::Less,
            },

            Value::UnsignedInteger(unsigned_integer) => match other {
                Self::Null | Self::Integer(_) => Ordering::Greater,
                Self::UnsignedInteger(other_unsigned_integer) => unsigned_integer.cmp(other_unsigned_integer),
                _ => Ordering::Less,
            },

            Value::Float(float) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) => Ordering::Greater,
                Self::Float(other_float) => OrderedFloat::from(*float).cmp(&OrderedFloat::from(*other_float)),
                _ => Ordering::Less,
            },

            Value::Boolean(boolean) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) => Ordering::Greater,
                Self::Boolean(other_boolean) => boolean.cmp(other_boolean),
                _ => Ordering::Less,
            },

            Value::Text(text) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) | Self::Boolean(_) => {
                    Ordering::Greater
                }

                Self::Text(other_text) => text.cmp(other_text),

                _ => Ordering::Less,
            },

            Value::Bytes(bytes) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_) => Ordering::Greater,

                Self::Bytes(other_bytes) => bytes.cmp(other_bytes),

                _ => Ordering::Less,
            },

            Value::NestedList(nested_list) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Bytes(_) => Ordering::Greater,

                Self::NestedList(other_nested_list) => nested_list.to_list().cmp(other_nested_list.to_list()),

                _ => Ordering::Less,
            },

            Value::NestedMap(nested_map) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Bytes(_)
                | Self::NestedList(_) => Ordering::Greater,

                Self::NestedMap(other_nested_map) => nested_map.to_map().cmp(other_nested_map.to_map()),
            },
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => fmt::Display::fmt("null", formatter),

            Self::Integer(integer) => fmt::Display::fmt(integer, formatter),

            Self::UnsignedInteger(unsigned_integer) => fmt::Display::fmt(unsigned_integer, formatter),

            Self::Float(float) => fmt::Display::fmt(float, formatter),

            Self::Boolean(boolean) => fmt::Display::fmt(boolean, formatter),

            Self::Text(text) => fmt::Debug::fmt(text, formatter),

            Self::Bytes(bytes) => write!(formatter, "{} bytes", bytes.len()),

            Self::NestedList(nested_list) => fmt::Display::fmt(nested_list.to_list(), formatter),

            Self::NestedMap(nested_map) => fmt::Display::fmt(nested_map.to_map(), formatter),
        }
    }
}
