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

            Any::NestedList(nested_list) => {
                let list: Vec<Self> = nested_list.get().into_iter().map(|item| item.into()).collect();
                list.into()
            }

            Any::NestedMap(nested_map) => {
                let map: BTreeMap<Self, Self> =
                    nested_map.get().into_iter().map(|(key, value)| (key.into(), value.into())).collect();
                map.into()
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

            data::Any::NestedList(nested_list) => {
                let vector: Vec<Any> = nested_list.to_list().inner.iter().map(|item| item.clone().into()).collect();
                Self::NestedList(NestedList::new(vector))
            }

            data::Any::NestedMap(nested_map) => {
                let key_value_pairs: Vec<(Any, Any)> = nested_map
                    .to_map()
                    .inner
                    .iter()
                    .map(|(key, value)| (key.clone().into(), value.clone().into()))
                    .collect();
                Self::NestedMap(NestedMap::new(key_value_pairs))
            }
        }
    }
}

// Kind

impl From<Kind> for dispatch_bindings::Kind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Group => Self::Group,
            Kind::NodeTemplate => Self::NodeTemplate,
            Kind::RelationshipTemplate => Self::RelationshipTemplate,
            Kind::Node => Self::Node,
            Kind::Relationship => Self::Relationship,
        }
    }
}

impl From<dispatch_bindings::Kind> for Kind {
    fn from(kind: dispatch_bindings::Kind) -> Self {
        match kind {
            dispatch_bindings::Kind::Group => Self::Group,
            dispatch_bindings::Kind::NodeTemplate => Self::NodeTemplate,
            dispatch_bindings::Kind::RelationshipTemplate => Self::RelationshipTemplate,
            dispatch_bindings::Kind::Node => Self::Node,
            dispatch_bindings::Kind::Relationship => Self::Relationship,
        }
    }
}

// Id

impl From<Id> for dispatch_bindings::Id {
    fn from(id: Id) -> Self {
        Self { kind: id.kind.into(), scope: id.scope, id: id.id }
    }
}

impl From<dispatch_bindings::Id> for Id {
    fn from(id: dispatch_bindings::Id) -> Self {
        Self { kind: id.kind.into(), scope: id.scope, id: id.id }
    }
}
