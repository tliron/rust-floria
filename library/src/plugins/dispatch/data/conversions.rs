use super::super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

// Kind

impl From<dispatch::Kind> for Kind {
    fn from(kind: dispatch::Kind) -> Self {
        match kind {
            dispatch::Kind::Class => Self::Class,
            dispatch::Kind::VertexTemplate => Self::VertexTemplate,
            dispatch::Kind::EdgeTemplate => Self::EdgeTemplate,
            dispatch::Kind::Vertex => Self::Vertex,
            dispatch::Kind::Edge => Self::Edge,
        }
    }
}

impl From<Kind> for dispatch::Kind {
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

// ID

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
