use super::super::{super::data::*, bindings::floria::plugins::floria as host};

impl From<host::Id> for ID {
    fn from(id: host::Id) -> Self {
        let namespace = id.namespace.into_iter().map(|segment| segment.into()).collect();
        Self::new_for(id.kind.into(), namespace, id.id.into())
    }
}

impl From<ID> for host::Id {
    fn from(id: ID) -> Self {
        let namespace = id.namespace.into_iter().map(|segment| segment.into()).collect();
        Self { kind: id.kind.into(), namespace: namespace, id: id.id.into() }
    }
}
