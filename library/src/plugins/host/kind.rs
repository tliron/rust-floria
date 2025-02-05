use super::super::{super::data::*, bindings::floria::plugins::floria as host};

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
