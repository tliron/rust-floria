use {
    kutil::{
        cli::depict::*,
        std::{error::*, *},
    },
    std::io,
};

//
// Kind
//

/// Kind.
#[derive(Clone, Copy, Debug, Display, Eq, FromStr, Hash, PartialEq)]
pub enum Kind {
    /// Class.
    Class,

    /// Vertex template.
    VertexTemplate,

    /// Edge template.
    EdgeTemplate,

    /// Vertex.
    Vertex,

    /// Edge.
    Edge,
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
