use {
    kutil::{cli::depict::*, message_error, std::*},
    std::io,
};

//
// Kind
//

/// Kind.
#[derive(Clone, Copy, Debug, Display, Eq, FromStr, Hash, PartialEq)]
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

impl Depict for Kind {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_meta(writer, self)
    }
}

//
// UnknownKindError
//

message_error!(UnknownKindError, "unknown kind");
