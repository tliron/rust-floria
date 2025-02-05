use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

impl From<dispatch::Id> for ID {
    fn from(id: dispatch::Id) -> Self {
        let directory = id.directory.into_iter().map(|segment| segment.into()).collect();
        Self::new_for(id.kind.into(), directory, id.id.into())
    }
}

impl From<ID> for dispatch::Id {
    fn from(id: ID) -> Self {
        let directory = id.directory.into_iter().map(|segment| segment.into()).collect();
        Self { kind: id.kind.into(), directory, id: id.id.into() }
    }
}
