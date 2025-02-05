use super::{kind::*, scope::*};

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

    /// Scope.
    pub scope: Scope,

    /// ID.
    pub id: ByteString,
}

impl ID {
    /// Constructor
    pub fn new(kind: Kind, scope: Scope) -> Self {
        Self::new_for(kind, scope, Default::default())
    }

    /// Constructor
    pub fn new_for(kind: Kind, scope: Scope, id: ByteString) -> Self {
        Self { kind, scope, id }
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

    /// Parse [Scope].
    pub fn parse_scope(scope: &str) -> Scope {
        scope.split(":").map(|segment| segment.into()).collect()
    }

    /// To [Scope].
    pub fn to_scope(&self) -> Scope {
        let mut scope = self.scope.clone();
        scope.push(self.id.clone());
        scope
    }
}

impl Depict for ID {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        for segment in &self.scope {
            write!(writer, "{}", context.theme.name_style.remove_all_effects().style(segment))?;
            context.theme.write_delimiter(writer, ":")?;
        }
        write!(writer, "{}", context.theme.name_style.bold().style(&self.id))
    }
}

impl fmt::Display for ID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.scope {
            write!(formatter, "{}:", segment)?;
        }
        write!(formatter, "{}", self.id)
    }
}
