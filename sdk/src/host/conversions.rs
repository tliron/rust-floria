use super::super::{data, dispatch_bindings, floria_bindings::*};

use std::collections::*;

// Any

impl From<Any> for data::Any {
    fn from(value: Any) -> Self {
        match value {
            Any::Null => Self::Null,
            Any::Integer(integer) => Self::Integer(integer),
            Any::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(unsigned_integer),
            Any::Float(float) => Self::Float(float),
            Any::Boolean(boolean) => Self::Boolean(boolean),
            Any::Text(text) => Self::Text(text),
            Any::Blob(blob) => Self::Blob(blob),

            Any::AnyList(any_list) => {
                let list: Vec<Self> = any_list.get().into_iter().map(|item| item.into()).collect();
                list.into()
            }

            Any::AnyMap(any_map) => {
                let map: BTreeMap<Self, Self> =
                    any_map.get().into_iter().map(|(key, value)| (key.into(), value.into())).collect();
                map.into()
            }

            Any::AnyCall(any_call) => {
                let (name, arguments) = any_call.get();
                let arguments: Vec<_> = arguments.into_iter().map(|item| item.into()).collect();
                data::Call { name, arguments }.into()
            }
        }
    }
}

impl From<data::Any> for Any {
    fn from(value: data::Any) -> Self {
        match value {
            data::Any::Null => Self::Null,
            data::Any::Integer(integer) => Self::Integer(integer),
            data::Any::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(unsigned_integer),
            data::Any::Float(float) => Self::Float(float),
            data::Any::Boolean(boolean) => Self::Boolean(boolean),
            data::Any::Text(text) => Self::Text(text),
            data::Any::Blob(blob) => Self::Blob(blob),

            data::Any::AnyList(any_list) => {
                let list: Vec<_> = any_list.to_list().inner.iter().map(|item| item.clone().into()).collect();
                Self::AnyList(AnyList::new(list))
            }

            data::Any::AnyMap(any_map) => {
                let key_value_pairs: Vec<_> = any_map
                    .to_map()
                    .inner
                    .iter()
                    .map(|(key, value)| (key.clone().into(), value.clone().into()))
                    .collect();
                Self::AnyMap(AnyMap::new(key_value_pairs))
            }

            data::Any::AnyCall(any_call) => {
                let call = any_call.to_call();
                let arguments: Vec<_> = call.arguments.iter().map(|item| item.clone().into()).collect();
                Self::AnyCall(AnyCall::new(&call.name, arguments))
            }
        }
    }
}

// Kind

impl From<Kind> for dispatch_bindings::Kind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Class => Self::Class,
            Kind::VertexTemplate => Self::VertexTemplate,
            Kind::EdgeTemplate => Self::EdgeTemplate,
            Kind::Vertex => Self::Vertex,
            Kind::Edge => Self::Edge,
        }
    }
}

impl From<dispatch_bindings::Kind> for Kind {
    fn from(kind: dispatch_bindings::Kind) -> Self {
        match kind {
            dispatch_bindings::Kind::Class => Self::Class,
            dispatch_bindings::Kind::VertexTemplate => Self::VertexTemplate,
            dispatch_bindings::Kind::EdgeTemplate => Self::EdgeTemplate,
            dispatch_bindings::Kind::Vertex => Self::Vertex,
            dispatch_bindings::Kind::Edge => Self::Edge,
        }
    }
}

// Id

impl From<Id> for dispatch_bindings::Id {
    fn from(id: Id) -> Self {
        Self { kind: id.kind.into(), directory: id.directory, id: id.id }
    }
}

impl From<dispatch_bindings::Id> for Id {
    fn from(id: dispatch_bindings::Id) -> Self {
        Self { kind: id.kind.into(), directory: id.directory, id: id.id }
    }
}

// Site

impl From<Site> for dispatch_bindings::Site {
    fn from(site: Site) -> Self {
        Self { id: site.id.into(), path: site.path }
    }
}

impl From<dispatch_bindings::Site> for Site {
    fn from(site: dispatch_bindings::Site) -> Self {
        Self { id: site.id.into(), path: site.path }
    }
}
