use super::{kind::*, prefix::*};

use {
    kutil::{cli::depict::*, std::zerocopy::*},
    std::{fmt, io},
};

//
// ID
//

/// ID.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ID {
    /// Kind.
    pub kind: Kind,

    /// Prefix.
    pub prefix: Prefix,

    /// ID.
    pub id: ByteString,
}

impl ID {
    /// Constructor
    pub fn new(kind: Kind, prefix: Prefix) -> Self {
        Self::new_for(kind, prefix, Default::default())
    }

    /// Constructor
    pub fn new_for(kind: Kind, prefix: Prefix, id: ByteString) -> Self {
        Self { kind, prefix, id }
    }

    /// Parse.
    pub fn parse(kind: Kind, id: &str) -> Self {
        let segments: Vec<&str> = id.split(":").collect();
        let length = segments.len();
        if length > 0 {
            Self::new_for(
                kind,
                segments[..length - 1].iter().map(|segment| (*segment).into()).collect(),
                segments[length - 1].into(),
            )
        } else {
            Self::new_for(kind, Default::default(), id.into())
        }
    }

    /// Parse [Prefix].
    pub fn parse_prefix(prefix: &str) -> Prefix {
        prefix.split(":").map(|segment| segment.into()).collect()
    }

    /// To [Prefix].
    pub fn to_prefix(&self) -> Prefix {
        let mut prefix = self.prefix.clone();
        prefix.push(self.id.clone());
        prefix
    }
}

impl Depict for ID {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        for segment in &self.prefix {
            write!(writer, "{}", context.theme.name_style.remove_all_effects().style(segment))?;
            context.theme.write_delimiter(writer, ":")?;
        }
        write!(writer, "{}", context.theme.name_style.bold().style(&self.id))
    }
}

impl fmt::Display for ID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.prefix {
            write!(formatter, "{}:", segment)?;
        }
        write!(formatter, "{}", self.id)
    }
}
