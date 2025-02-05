use super::{kind::*, namespace::*};

use {
    bytestring::*,
    kutil_cli::debug::*,
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

    /// Namespace.
    pub namespace: Namespace,

    /// ID.
    pub id: ByteString,
}

impl ID {
    /// Constructor
    pub fn new(kind: Kind, namespace: Namespace) -> Self {
        Self::new_for(kind, namespace, Default::default())
    }

    /// Constructor
    pub fn new_for(kind: Kind, namespace: Namespace, id: ByteString) -> Self {
        Self { kind, namespace, id }
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

    /// Parse namespace.
    pub fn parse_namespace(namespace: &str) -> Namespace {
        namespace.split(":").map(|segment| segment.into()).collect()
    }

    /// To namespace.
    pub fn to_namespace(&self) -> Namespace {
        let mut namespace = self.namespace.clone();
        namespace.push(self.id.clone());
        namespace
    }
}

impl Debuggable for ID {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        for segment in &self.namespace {
            write!(writer, "{}", context.theme.name_style.remove_all_effects().style(segment))?;
            context.theme.write_delimiter(writer, ":")?;
        }
        write!(writer, "{}", context.theme.name_style.bold().style(&self.id))
    }
}

impl fmt::Display for ID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.namespace {
            write!(formatter, "{}:", segment)?;
        }
        write!(formatter, "{}", self.id)
    }
}
