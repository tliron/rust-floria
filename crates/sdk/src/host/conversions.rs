use super::super::{dispatch_bindings, floria_bindings::*, normal};

use std::collections::*;

// Value

impl From<Value> for normal::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,

            Value::Integer(integer) => Self::Integer(integer),

            Value::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(unsigned_integer),

            Value::Float(float) => Self::Float(float),

            Value::Boolean(boolean) => Self::Boolean(boolean),

            Value::Text(text) => Self::Text(text),

            Value::Bytes(bytes) => Self::Bytes(bytes),

            Value::NestedList(nested_list) => {
                let list: Vec<Self> = nested_list.get().into_iter().map(|v| v.into()).collect();
                list.into()
            }

            Value::NestedMap(nested_map) => {
                let map: BTreeMap<Self, Self> =
                    nested_map.get().into_iter().map(|(k, v)| (k.into(), v.into())).collect();
                map.into()
            }
        }
    }
}

impl From<normal::Value> for Value {
    fn from(value: normal::Value) -> Self {
        match value {
            normal::Value::Null => Self::Null,

            normal::Value::Integer(integer) => Self::Integer(integer),

            normal::Value::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(unsigned_integer),

            normal::Value::Float(float) => Self::Float(float),

            normal::Value::Boolean(boolean) => Self::Boolean(boolean),

            normal::Value::Text(text) => Self::Text(text),

            normal::Value::Bytes(bytes) => Self::Bytes(bytes),

            normal::Value::NestedList(nested_list) => {
                let vector: Vec<Value> = nested_list.to_list().value.iter().map(|v| v.clone().into()).collect();
                Self::NestedList(NestedList::new(vector))
            }

            normal::Value::NestedMap(nested_map) => {
                let key_value_pairs: Vec<(Value, Value)> =
                    nested_map.to_map().value.iter().map(|(k, v)| (k.clone().into(), v.clone().into())).collect();
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
        Self { kind: id.kind.into(), namespace: id.namespace, id: id.id }
    }
}

impl From<dispatch_bindings::Id> for Id {
    fn from(id: dispatch_bindings::Id) -> Self {
        Self { kind: id.kind.into(), namespace: id.namespace, id: id.id }
    }
}
