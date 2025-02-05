use super::{super::dispatch_bindings::*, prefix::*};

use std::fmt;

//
// Id
//

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        (self.kind == other.kind) && (self.prefix == other.prefix) && (self.id == other.id)
    }
}

impl Id {
    /// Constructor
    pub fn new(kind: Kind, prefix: Prefix) -> Self {
        Self::new_for(kind, prefix, Default::default())
    }

    /// Constructor
    pub fn new_for(kind: Kind, prefix: Prefix, id: String) -> Self {
        Self { kind, prefix, id }
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

    /// Parse prefix.
    pub fn parse_prefix(prefix: &str) -> Prefix {
        prefix.split(":").map(|segment| segment.into()).collect()
    }

    /// To prefix.
    pub fn to_prefix(&self) -> Prefix {
        let mut prefix = self.prefix.clone();
        prefix.push(self.id.clone());
        prefix
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.prefix {
            write!(formatter, "{}:", segment)?;
        }
        write!(formatter, "{}", self.id)
    }
}
