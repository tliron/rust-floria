use super::super::{super::data::*, bindings::floria::plugins::floria as host};

impl From<host::Kind> for Kind {
    fn from(kind: host::Kind) -> Self {
        match kind {
            host::Kind::Group => Self::Group,
            host::Kind::NodeTemplate => Self::NodeTemplate,
            host::Kind::RelationshipTemplate => Self::RelationshipTemplate,
            host::Kind::Node => Self::Node,
            host::Kind::Relationship => Self::Relationship,
        }
    }
}

impl From<Kind> for host::Kind {
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
