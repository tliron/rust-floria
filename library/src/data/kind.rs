use {
    kutil::{cli::debug::*, message_error, std::*},
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

impl Debuggable for Kind {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
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
