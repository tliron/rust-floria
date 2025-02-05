use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

use std::fmt;

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

impl fmt::Display for dispatch::Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            match self {
                Self::Class => "Class",
                Self::VertexTemplate => "VertexTemplate",
                Self::EdgeTemplate => "EdgeTemplate",
                Self::Vertex => "Vertex",
                Self::Edge => "Edge",
            },
            formatter,
        )
    }
}
