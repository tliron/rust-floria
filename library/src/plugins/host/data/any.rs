use super::super::super::bindings::floria::plugins::floria::*;

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
            Self::AnyList(resource) => Self::AnyList(Resource::new_own(resource.rep())),
            Self::AnyMap(resource) => Self::AnyMap(Resource::new_own(resource.rep())),
            Self::AnyCall(resource) => Self::AnyCall(Resource::new_own(resource.rep())),
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
            Self::AnyList(resource) => resource.rep().hash(state),
            Self::AnyMap(resource) => resource.rep().hash(state),
            Self::AnyCall(resource) => resource.rep().hash(state),
        }
    }
}

impl PartialEq for Any {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Integer(integer), Self::Integer(other_integer)) => integer == other_integer,
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer == other_unsigned_integer
            }
            (Self::Float(float), Self::Float(other_float)) => float == other_float,
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean == other_boolean,
            (Self::Text(text), Self::Text(other_text)) => text == other_text,
            (Self::Blob(blob), Self::Blob(other_blob)) => blob == other_blob,

            // Important: these are not content comparisons!
            (Self::AnyList(resource), Self::AnyList(other_resource)) => resource.rep() == other_resource.rep(),
            (Self::AnyMap(resource), Self::AnyMap(other_resource)) => resource.rep() == other_resource.rep(),
            (Self::AnyCall(resource), Self::AnyCall(other_resource)) => resource.rep() == other_resource.rep(),

            _ => false,
        }
    }
}

impl Eq for Any {}

impl PartialOrd for Any {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Integer(integer), Self::Integer(other_integer)) => integer.partial_cmp(other_integer),
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer.partial_cmp(other_unsigned_integer)
            }
            (Self::Float(float), Self::Float(other_float)) => float.partial_cmp(other_float),
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean.partial_cmp(other_boolean),
            (Self::Text(text), Self::Text(other_text)) => text.partial_cmp(other_text),
            (Self::Blob(blob), Self::Blob(other_blob)) => blob.partial_cmp(other_blob),

            // Important: these are not content comparisons!
            (Self::AnyList(resource), Self::AnyList(other_resource)) => {
                if resource.rep() == other_resource.rep() {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            (Self::AnyMap(resource), Self::AnyMap(other_resource)) => {
                if resource.rep() == other_resource.rep() {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            (Self::AnyCall(resource), Self::AnyCall(other_resource)) => {
                if resource.rep() == other_resource.rep() {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            _ => None,
        }
    }
}

impl Ord for Any {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Null, Self::Null) => Ordering::Equal,
            (Self::Integer(integer), Self::Integer(other_integer)) => integer.cmp(other_integer),
            (Self::UnsignedInteger(unsigned_integer), Self::UnsignedInteger(other_unsigned_integer)) => {
                unsigned_integer.cmp(other_unsigned_integer)
            }
            (Self::Float(float), Self::Float(other_float)) => {
                OrderedFloat::from(*float).cmp(&OrderedFloat::from(*other_float))
            }
            (Self::Boolean(boolean), Self::Boolean(other_boolean)) => boolean.cmp(other_boolean),
            (Self::Text(text), Self::Text(other_text)) => text.cmp(other_text),
            (Self::Blob(blob), Self::Blob(other_blob)) => blob.cmp(other_blob),

            // Important: these are not content comparisons!
            (Self::AnyList(resource), Self::AnyList(other_resource)) => resource.rep().cmp(&other_resource.rep()),
            (Self::AnyMap(resource), Self::AnyMap(other_resource)) => resource.rep().cmp(&other_resource.rep()),
            (Self::AnyCall(resource), Self::AnyCall(other_resource)) => resource.rep().cmp(&other_resource.rep()),

            (Self::Null, _) => Ordering::Less,

            (Self::Integer(_), Self::Null) => Ordering::Greater,
            (Self::Integer(_), _) => Ordering::Less,

            (Self::UnsignedInteger(_), Self::Null | Self::Integer(_)) => Ordering::Greater,
            (Self::UnsignedInteger(_), _) => Ordering::Less,

            (Self::Float(_), Self::Null | Self::Integer(_) | Self::UnsignedInteger(_)) => Ordering::Greater,
            (Self::Float(_), _) => Ordering::Less,

            (Self::Boolean(_), Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_)) => {
                Ordering::Greater
            }
            (Self::Boolean(_), _) => Ordering::Less,

            (
                Self::Text(_),
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) | Self::Boolean(_),
            ) => Ordering::Greater,
            (Self::Text(_), _) => Ordering::Less,

            (
                Self::Blob(_),
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_),
            ) => Ordering::Greater,
            (Self::Blob(_), _) => Ordering::Less,

            (
                Self::AnyList(_),
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_),
            ) => Ordering::Greater,
            (Self::AnyList(_), _) => Ordering::Less,

            (
                Self::AnyMap(_),
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Blob(_)
                | Self::AnyList(_),
            ) => Ordering::Greater,
            (Self::AnyMap(_), _) => Ordering::Less,

            (Self::AnyCall(_), _) => Ordering::Less,
        }
    }
}
