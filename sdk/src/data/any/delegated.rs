use super::super::super::dispatch_bindings::*;

use {
    ordered_float::*,
    std::{cmp::*, fmt, hash::*},
};

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
            (Self::AnyList(any_list), Self::AnyList(other_any_list)) => any_list.to_list() == other_any_list.to_list(),
            (Self::AnyMap(any_map), Self::AnyMap(other_any_map)) => any_map.to_map() == other_any_map.to_map(),
            (Self::AnyCall(any_call), Self::AnyCall(other_any_call)) => any_call.to_call() == other_any_call.to_call(),

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
            (Self::AnyList(any_list), Self::AnyList(other_any_list)) => {
                any_list.to_list().partial_cmp(other_any_list.to_list())
            }
            (Self::AnyMap(any_map), Self::AnyMap(other_any_map)) => {
                any_map.to_map().partial_cmp(other_any_map.to_map())
            }
            (Self::AnyCall(any_call), Self::AnyCall(other_any_call)) => {
                any_call.to_call().partial_cmp(other_any_call.to_call())
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
            (Self::AnyList(any_list), Self::AnyList(other_any_list)) => {
                any_list.to_list().cmp(other_any_list.to_list())
            }
            (Self::AnyMap(any_map), Self::AnyMap(other_any_map)) => any_map.to_map().cmp(other_any_map.to_map()),
            (Self::AnyCall(any_call), Self::AnyCall(other_any_call)) => {
                any_call.to_call().cmp(other_any_call.to_call())
            }

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

impl Hash for Any {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Null => {
                state.write_u8(1);
                ().hash(state);
            }

            Self::Integer(integer) => {
                state.write_u8(2);
                integer.hash(state);
            }

            Self::UnsignedInteger(unsigned_integer) => {
                state.write_u8(3);
                unsigned_integer.hash(state);
            }

            Self::Float(float) => {
                state.write_u8(4);
                OrderedFloat::from(*float).hash(state);
            }

            Self::Boolean(boolean) => {
                state.write_u8(5);
                boolean.hash(state);
            }

            Self::Text(text) => {
                state.write_u8(6);
                text.hash(state);
            }

            Self::Blob(blob) => {
                state.write_u8(7);
                blob.hash(state);
            }

            Self::AnyList(any_list) => {
                state.write_u8(8);
                any_list.to_list().hash(state);
            }

            Self::AnyMap(any_map) => {
                state.write_u8(9);
                any_map.to_map().hash(state);
            }

            Self::AnyCall(any_call) => {
                state.write_u8(10);
                any_call.to_call().hash(state);
            }
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
            Self::AnyList(any_list) => fmt::Display::fmt(any_list.to_list(), formatter),
            Self::AnyMap(any_map) => fmt::Display::fmt(any_map.to_map(), formatter),
            Self::AnyCall(any_call) => fmt::Display::fmt(any_call.to_call(), formatter),
        }
    }
}
