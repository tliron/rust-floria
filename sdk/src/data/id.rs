use super::{super::dispatch_bindings::*, directory::*};

use std::fmt;

//
// Id
//

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        (self.kind == other.kind) && (self.directory == other.directory) && (self.id == other.id)
    }
}

impl Id {
    /// Constructor
    pub fn new(kind: Kind, directory: Directory) -> Self {
        Self::new_for(kind, directory, Default::default())
    }

    /// Constructor
    pub fn new_for(kind: Kind, directory: Directory, id: String) -> Self {
        Self { kind, directory, id }
    }

    /// Constructor
    pub fn new_from(any: &Any) -> Option<Self> {
        if let Some(id) = any.get(&"id".into())
            && let Any::Text(id) = id
            && let Some(kind) = any.get(&"kind".into())
            && let Any::Text(kind) = kind
            && let Ok(kind) = Kind::try_from(kind.as_str())
        {
            return Some(Self::parse(kind, id));
        }

        None
    }

    /// Parse.
    pub fn parse(kind: Kind, id: &str) -> Self {
        let segments: Vec<&str> = id.split(":").collect();
        let length = segments.len();
        if length > 0 {
            Self::new_for(
                kind,
                segments[..length - 1].iter().map(|segment| segment.to_string()).collect(),
                segments[length - 1].into(),
            )
        } else {
            Self::new_for(kind, Default::default(), id.into())
        }
    }

    /// Parse [Directory].
    pub fn parse_directory(directory: &str) -> Directory {
        directory.split(":").map(|segment| segment.into()).collect()
    }

    /// To [Directory].
    pub fn to_directory(&self) -> Directory {
        let mut directory = self.directory.clone();
        directory.push(self.id.clone());
        directory
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.directory {
            write!(formatter, "{}:", segment)?;
        }
        write!(formatter, "{}", self.id)
    }
}
