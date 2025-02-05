use super::super::dispatch_bindings::*;

use std::fmt;

//
// Kind
//

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Class, Self::Class)
                | (Self::VertexTemplate, Self::VertexTemplate)
                | (Self::EdgeTemplate, Self::EdgeTemplate)
                | (Self::Vertex, Self::Vertex)
                | (Self::Edge, Self::Edge)
        )
    }
}

impl fmt::Display for Kind {
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

impl TryFrom<&str> for Kind {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Class" => Ok(Self::Class),
            "VertexTemplate" => Ok(Self::VertexTemplate),
            "EdgeTemplate" => Ok(Self::EdgeTemplate),
            "Vertex" => Ok(Self::Vertex),
            "Edge" => Ok(Self::Edge),
            _ => Err(format!("unsupported kind: {}", value)),
        }
    }
}
