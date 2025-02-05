use super::super::bindings::floria::plugins::floria::*;

use {
    ordered_float::*,
    std::{cmp::*, hash::*},
    wasmtime::component::Resource,
};

impl Clone for Any {
    /// Clone resource.
    fn clone(&self) -> Any {
        match self {
            Self::Null => Self::Null,
            Self::Integer(integer) => Self::Integer(*integer),
            Self::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),
            Self::Float(float) => Self::Float(*float),
            Self::Boolean(boolean) => Self::Boolean(*boolean),
            Self::Text(text) => Self::Text(text.clone()),
            Self::Blob(blob) => Self::Blob(blob.clone()),
            // TODO: own or borrow?
            Self::NestedList(resource) => Self::NestedList(Resource::new_own(resource.rep())),
            Self::NestedMap(resource) => Self::NestedMap(Resource::new_own(resource.rep())),
        }
    }
}

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
            Self::NestedList(nested_map) => nested_map.rep().hash(state),
            Self::NestedMap(value_map) => value_map.rep().hash(state),
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
                // Important: this is not a content comparison!
                if let Self::NestedList(other_nested_list) = other {
                    return nested_list.rep() == other_nested_list.rep();
                }
            }

            Self::NestedMap(nested_map) => {
                // Important: this is not a content comparison!
                if let Self::NestedMap(other_nested_map) = other {
                    return nested_map.rep() == other_nested_map.rep();
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
                // Important: this is not a content comparison!
                if let Self::NestedList(other_nested_list) = other
                    && nested_list.rep() == other_nested_list.rep()
                {
                    return Some(Ordering::Equal);
                }
            }

            Self::NestedMap(nested_map) => {
                // Important: this is not a content comparison!
                if let Self::NestedMap(other_nested_map) = other
                    && nested_map.rep() == other_nested_map.rep()
                {
                    return Some(Ordering::Equal);
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

            Any::Integer(integer) => match other {
                Self::Null => Ordering::Greater,
                Self::Integer(other_integer) => integer.cmp(other_integer),
                _ => Ordering::Less,
            },

            Any::UnsignedInteger(unsigned_integer) => match other {
                Self::Null | Self::Integer(_) => Ordering::Greater,
                Self::UnsignedInteger(other_unsigned_integer) => unsigned_integer.cmp(other_unsigned_integer),
                _ => Ordering::Less,
            },

            Any::Float(float) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) => Ordering::Greater,
                Self::Float(other_float) => OrderedFloat::from(*float).cmp(&OrderedFloat::from(*other_float)),
                _ => Ordering::Less,
            },

            Any::Boolean(boolean) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) => Ordering::Greater,
                Self::Boolean(other_boolean) => boolean.cmp(other_boolean),
                _ => Ordering::Less,
            },

            Any::Text(text) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) | Self::Boolean(_) => {
                    Ordering::Greater
                }

                Self::Text(other_text) => text.cmp(other_text),

                _ => Ordering::Less,
            },

            Any::Blob(blob) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_) => Ordering::Greater,

                Self::Blob(other_blob) => blob.cmp(other_blob),

                _ => Ordering::Less,
            },

            Any::NestedList(nested_list) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_) => Ordering::Greater,

                // Important: this is not a content comparison!
                Self::NestedList(other_nested_list) => nested_list.rep().cmp(&other_nested_list.rep()),

                _ => Ordering::Less,
            },

            Any::NestedMap(nested_map) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_)
                | Self::NestedList(_) => Ordering::Greater,

                // Important: this is not a content comparison!
                Self::NestedMap(other_nested_map) => nested_map.rep().cmp(&other_nested_map.rep()),
            },
        }
    }
}
