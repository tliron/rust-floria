use super::super::{super::data::*, bindings::exports::floria::plugins::dispatch};

use std::fmt;

impl From<dispatch::Kind> for Kind {
    fn from(kind: dispatch::Kind) -> Self {
        match kind {
            dispatch::Kind::Group => Self::Group,
            dispatch::Kind::NodeTemplate => Self::NodeTemplate,
            dispatch::Kind::RelationshipTemplate => Self::RelationshipTemplate,
            dispatch::Kind::Node => Self::Node,
            dispatch::Kind::Relationship => Self::Relationship,
        }
    }
}

impl From<Kind> for dispatch::Kind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Group => Self::Group,
            Kind::NodeTemplate => Self::NodeTemplate,
            Kind::RelationshipTemplate => Self::RelationshipTemplate,
            Kind::Node => Self::Node,
            Kind::Relationship => Self::Relationship,
        }
    }
}

impl fmt::Display for dispatch::Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            match self {
                Self::Group => "Group",
                Self::NodeTemplate => "NodeTemplate",
                Self::RelationshipTemplate => "RelationshipTemplate",
                Self::Node => "Node",
                Self::Relationship => "Relationship",
            },
            formatter,
        )
    }
}
