use std::fmt;

//
// Kind
//

/// Kind.
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    /// None.
    #[default]
    None,

    /// Type.
    Type,

    /// Node template.
    NodeTemplate,

    /// Relationship template.
    RelationshipTemplate,

    /// Node.
    Node,

    /// Relationship.
    Relationship,
}

impl fmt::Display for Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            match self {
                Self::None => "None",
                Self::Type => "Type",
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
            "None" => Ok(Self::None),
            "Type" => Ok(Self::Type),
            "NodeTemplate" => Ok(Self::NodeTemplate),
            "RelationshipTemplate" => Ok(Self::RelationshipTemplate),
            "Node" => Ok(Self::Node),
            "Relationship" => Ok(Self::Relationship),
            _ => Err(format!("unsupported kind: {}", value)),
        }
    }
}
