use super::super::{super::data::*, bindings::floria::plugins::floria as host};

impl From<host::Id> for ID {
    fn from(id: host::Id) -> Self {
        Self::new_for(id.kind.into(), id.namespace, id.id)
    }
}

impl From<ID> for host::Id {
    fn from(id: ID) -> Self {
        Self { kind: id.kind.into(), namespace: id.namespace, id: id.id }
    }
}
