use super::super::dispatch_bindings::*;

use std::fmt;

//
// Kind
//

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Group, Self::Group)
                | (Self::NodeTemplate, Self::NodeTemplate)
                | (Self::RelationshipTemplate, Self::RelationshipTemplate)
                | (Self::Node, Self::Node)
                | (Self::Relationship, Self::Relationship)
        )
    }
}

impl fmt::Display for Kind {
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

impl TryFrom<&str> for Kind {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Group" => Ok(Self::Group),
            "NodeTemplate" => Ok(Self::NodeTemplate),
            "RelationshipTemplate" => Ok(Self::RelationshipTemplate),
            "Node" => Ok(Self::Node),
            "Relationship" => Ok(Self::Relationship),
            _ => Err(format!("unsupported kind: {}", value)),
        }
    }
}
