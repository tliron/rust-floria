use {
    kutil_cli::debug::*,
    std::{fmt, io},
    thiserror::*,
};

//
// Kind
//

/// Kind.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    /// Group.
    Group,

    /// Node template.
    NodeTemplate,

    /// Relationship template.
    RelationshipTemplate,

    /// Node.
    Node,

    /// Relationship.
    Relationship,
}

impl Debuggable for Kind {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_meta(writer, self)
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
    type Error = UnknownKindError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Group" => Ok(Self::Group),
            "NodeTemplate" => Ok(Self::NodeTemplate),
            "RelationshipTemplate" => Ok(Self::RelationshipTemplate),
            "Node" => Ok(Self::Node),
            "Relationship" => Ok(Self::Relationship),
            _ => Err(UnknownKindError(value.into())),
        }
    }
}

//
// UnknownKindError
//

/// Uknown format error.
#[derive(Debug, Error)]
pub struct UnknownKindError(String);

impl UnknownKindError {
    /// Constructor.
    pub fn new(kind: &str) -> Self {
        Self(kind.into())
    }
}

impl fmt::Display for UnknownKindError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}
