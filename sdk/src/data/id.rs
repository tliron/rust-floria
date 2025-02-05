use super::{super::dispatch_bindings::*, scope::*};

use std::fmt;

//
// Id
//

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        (self.kind == other.kind) && (self.scope == other.scope) && (self.id == other.id)
    }
}

impl Id {
    /// Constructor
    pub fn new(kind: Kind, scope: Scope) -> Self {
        Self::new_for(kind, scope, Default::default())
    }

    /// Constructor
    pub fn new_for(kind: Kind, scope: Scope, id: String) -> Self {
        Self { kind, scope, id }
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

    /// Parse scope.
    pub fn parse_scope(scope: &str) -> Scope {
        scope.split(":").map(|segment| segment.into()).collect()
    }

    /// To scope.
    pub fn to_scope(&self) -> Scope {
        let mut scope = self.scope.clone();
        scope.push(self.id.clone());
        scope
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.scope {
            write!(formatter, "{}:", segment)?;
        }
        write!(formatter, "{}", self.id)
    }
}
