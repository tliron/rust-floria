use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

impl From<dispatch::Id> for ID {
    fn from(id: dispatch::Id) -> Self {
        let namespace = id.namespace.into_iter().map(|segment| segment.into()).collect();
        Self::new_for(id.kind.into(), namespace, id.id.into())
    }
}

impl From<ID> for dispatch::Id {
    fn from(id: ID) -> Self {
        let namespace = id.namespace.into_iter().map(|segment| segment.into()).collect();
        Self { kind: id.kind.into(), namespace: namespace, id: id.id.into() }
    }
}
