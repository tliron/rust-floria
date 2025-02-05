use super::super::super::{
    super::data::*,
    bindings::{exports::floria::plugins::dispatch, floria::plugins::floria as host},
};

// Kind

impl From<host::Kind> for Kind {
    fn from(kind: host::Kind) -> Self {
        match kind {
            host::Kind::Class => Self::Class,
            host::Kind::VertexTemplate => Self::VertexTemplate,
            host::Kind::EdgeTemplate => Self::EdgeTemplate,
            host::Kind::Vertex => Self::Vertex,
            host::Kind::Edge => Self::Edge,
        }
    }
}

impl From<Kind> for host::Kind {
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

impl From<host::Id> for ID {
    fn from(id: host::Id) -> Self {
        let directory = id.directory.into_iter().map(|segment| segment.into()).collect();
        Self::new_for(id.kind.into(), directory, id.id.into())
    }
}

impl From<ID> for host::Id {
    fn from(id: ID) -> Self {
        let directory = id.directory.into_iter().map(|segment| segment.into()).collect();
        Self { kind: id.kind.into(), directory, id: id.id.into() }
    }
}

// Site

impl From<host::Site> for dispatch::Site {
    fn from(site: host::Site) -> Self {
        Self::new(site.id.into(), site.path)
    }
}

impl From<dispatch::Site> for host::Site {
    fn from(site: dispatch::Site) -> Self {
        let id: ID = site.id.into();
        Self { id: id.into(), path: site.path }
    }
}
