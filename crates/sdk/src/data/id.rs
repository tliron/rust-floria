use super::{super::normal::*, kind::*, namespace::*};

use std::fmt;

//
// ID
//

/// ID.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ID {
    /// Kind.
    pub kind: Kind,

    /// Namespace.
    pub namespace: Namespace,

    /// ID.
    pub id: String,
}

impl ID {
    /// Constructor
    pub fn new(kind: Kind, namespace: Namespace) -> Self {
        Self::new_for(kind, namespace, String::new())
    }

    /// Constructor
    pub fn new_for(kind: Kind, namespace: Namespace, id: String) -> Self {
        Self { kind, namespace, id }
    }

    /// Constructor
    pub fn new_from(value: &Value) -> Option<Self> {
        if let Some(id) = value.get(&"id".into()) {
            if let Value::Text(id) = id {
                if let Some(kind) = value.get(&"kind".into()) {
                    if let Value::Text(kind) = kind {
                        if let Ok(kind) = Kind::try_from(kind.as_str()) {
                            return Some(Self::parse(kind, id));
                        }
                    }
                }
            }
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
                segments[..length - 1].iter().map(|s| s.to_string()).collect(),
                segments[length - 1].into(),
            )
        } else {
            Self::new_for(kind, Namespace::new(), id.into())
        }
    }

    /// Parse namespace.
    pub fn parse_namespace(namespace: &str) -> Namespace {
        namespace.split(":").map(|s| s.into()).collect()
    }

    /// To namespace.
    pub fn to_namespace(&self) -> Namespace {
        let mut namespace = self.namespace.clone();
        namespace.push(self.id.clone());
        namespace
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
