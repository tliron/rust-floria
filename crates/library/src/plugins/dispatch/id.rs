use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

impl From<dispatch::Id> for ID {
    fn from(id: dispatch::Id) -> Self {
        Self::new_for(id.kind.into(), id.namespace, id.id)
    }
}

impl From<ID> for dispatch::Id {
    fn from(id: ID) -> Self {
        Self { kind: id.kind.into(), namespace: id.namespace, id: id.id }
    }
}
