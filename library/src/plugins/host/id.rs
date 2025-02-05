use super::super::{super::data::*, bindings::floria::plugins::floria as host};

impl From<host::Id> for ID {
    fn from(id: host::Id) -> Self {
        let prefix = id.prefix.into_iter().map(|segment| segment.into()).collect();
        Self::new_for(id.kind.into(), prefix, id.id.into())
    }
}

impl From<ID> for host::Id {
    fn from(id: ID) -> Self {
        let prefix = id.prefix.into_iter().map(|segment| segment.into()).collect();
        Self { kind: id.kind.into(), prefix, id: id.id.into() }
    }
}
