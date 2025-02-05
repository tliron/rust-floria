use super::super::super::dispatch_bindings::*;

use {
    ordered_float::*,
    std::{cmp::*, fmt, hash::*},
};

impl Hash for Any {
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
            Self::Blob(blob) => blob.hash(state),
            Self::NestedList(nested_list) => nested_list.to_list().hash(state),
            Self::NestedMap(nested_map) => nested_map.to_map().hash(state),
        }
    }
}

impl PartialEq for Any {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Null => return matches!(other, Self::Null),

            Self::Integer(integer) => {
                if let Self::Integer(other_integer) = other {
                    return integer == other_integer;
                }
            }

            Self::UnsignedInteger(unsigned_integer) => {
                if let Self::UnsignedInteger(other_unsigned_integer) = other {
                    return unsigned_integer == other_unsigned_integer;
                }
            }

            Self::Float(float) => {
                if let Self::Float(other_float) = other {
                    return float == other_float;
                }
            }

            Self::Boolean(boolean) => {
                if let Self::Boolean(other_boolean) = other {
                    return boolean == other_boolean;
                }
            }

            Self::Text(text) => {
                if let Self::Text(other_text) = other {
                    return text == other_text;
                }
            }

            Self::Blob(blob) => {
                if let Self::Blob(other_blob) = other {
                    return blob == other_blob;
                }
            }

            Self::NestedList(nested_list) => {
                if let Self::NestedList(other_nested_list) = other {
                    return nested_list.to_list() == other_nested_list.to_list();
                }
            }

            Self::NestedMap(nested_map) => {
                if let Self::NestedMap(other_nested_map) = other {
                    return nested_map.to_map() == other_nested_map.to_map();
                }
            }
        }

        false
    }
}

impl Eq for Any {}

impl PartialOrd for Any {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Null => {
                if matches!(other, Self::Null) {
                    return Some(Ordering::Equal);
                }
            }

            Self::Integer(integer) => {
                if let Self::Integer(other_integer) = other {
                    return integer.partial_cmp(other_integer);
                }
            }

            Self::UnsignedInteger(unsigned_integer) => {
                if let Self::UnsignedInteger(other_unsigned_integer) = other {
                    return unsigned_integer.partial_cmp(other_unsigned_integer);
                }
            }

            Self::Float(float) => {
                if let Self::Float(other_float) = other {
                    return float.partial_cmp(other_float);
                }
            }

            Self::Boolean(boolean) => {
                if let Self::Boolean(other_boolean) = other {
                    return boolean.partial_cmp(other_boolean);
                }
            }

            Self::Text(text) => {
                if let Self::Text(other_text) = other {
                    return text.partial_cmp(other_text);
                }
            }

            Self::Blob(blob) => {
                if let Self::Blob(other_blob) = other {
                    return blob.partial_cmp(other_blob);
                }
            }

            Self::NestedList(nested_list) => {
                if let Self::NestedList(other_nested_list) = other {
                    return nested_list.to_list().partial_cmp(other_nested_list.to_list());
                }
            }

            Self::NestedMap(nested_map) => {
                if let Self::NestedMap(other_nested_map) = other {
                    return nested_map.to_map().partial_cmp(other_nested_map.to_map());
                }
            }
        }

        None
    }
}

impl Ord for Any {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Null => match other {
                Self::Null => Ordering::Equal,
                _ => Ordering::Less,
            },

            Self::Integer(integer) => match other {
                Self::Null => Ordering::Greater,
                Self::Integer(other_integer) => integer.cmp(other_integer),
                _ => Ordering::Less,
            },

            Self::UnsignedInteger(unsigned_integer) => match other {
                Self::Null | Self::Integer(_) => Ordering::Greater,
                Self::UnsignedInteger(other_unsigned_integer) => unsigned_integer.cmp(other_unsigned_integer),
                _ => Ordering::Less,
            },

            Self::Float(float) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) => Ordering::Greater,
                Self::Float(other_float) => OrderedFloat::from(*float).cmp(&OrderedFloat::from(*other_float)),
                _ => Ordering::Less,
            },

            Self::Boolean(boolean) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) => Ordering::Greater,
                Self::Boolean(other_boolean) => boolean.cmp(other_boolean),
                _ => Ordering::Less,
            },

            Self::Text(text) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) | Self::Boolean(_) => {
                    Ordering::Greater
                }

                Self::Text(other_text) => text.cmp(other_text),

                _ => Ordering::Less,
            },

            Self::Blob(blob) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_) => Ordering::Greater,

                Self::Blob(other_blob) => blob.cmp(other_blob),

                _ => Ordering::Less,
            },

            Self::NestedList(nested_list) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_) => Ordering::Greater,

                Self::NestedList(other_nested_list) => nested_list.to_list().cmp(other_nested_list.to_list()),

                _ => Ordering::Less,
            },

            Self::NestedMap(nested_map) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_)
                | Self::NestedList(_) => Ordering::Greater,

                Self::NestedMap(other_nested_map) => nested_map.to_map().cmp(other_nested_map.to_map()),
            },
        }
    }
}

impl fmt::Display for Any {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => fmt::Display::fmt("null", formatter),
            Self::Integer(integer) => fmt::Display::fmt(integer, formatter),
            Self::UnsignedInteger(unsigned_integer) => fmt::Display::fmt(unsigned_integer, formatter),
            Self::Float(float) => fmt::Display::fmt(float, formatter),
            Self::Boolean(boolean) => fmt::Display::fmt(boolean, formatter),
            Self::Text(text) => fmt::Debug::fmt(text, formatter),
            Self::Blob(blob) => write!(formatter, "{} bytes", blob.len()),
            Self::NestedList(nested_list) => fmt::Display::fmt(nested_list.to_list(), formatter),
            Self::NestedMap(nested_map) => fmt::Display::fmt(nested_map.to_map(), formatter),
        }
    }
}
