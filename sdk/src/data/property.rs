use super::{super::dispatch_bindings::*, any::*};

use std::collections::*;

//
// Property
//

/// Property.
#[derive(Clone)]
pub struct Property<'own> {
    /// Name.
    pub name: String,

    /// Any.
    pub any: &'own Any,
}

impl<'own> Property<'own> {
    /// Constructor.
    pub fn new(name: String, any: &'own Any) -> Self {
        Self { name, any }
    }

    /// Value.
    pub fn value(&self) -> Option<&Any> {
        self.any.into_get("value")
    }

    /// Metadata.
    pub fn metadata(&self) -> Result<Option<&BTreeMap<Any, Any>>, String> {
        match self.any.into_get("metadata") {
            Some(metadata) => match metadata {
                Any::AnyMap(metadata) => Ok(Some(&metadata.to_map().inner)),
                _ => Err(format!("property {}: malformed \"metadata\", not a map", self.name)),
            },

            _ => Ok(None),
        }
    }

    /// Get metadata string.
    pub fn get_metadata_string(&self, name: &str) -> Result<Option<&str>, String> {
        if let Some(metadata) = self.metadata()?
            && let Some(vale) = metadata.get(&name.into())
            && let Any::Text(value) = vale
        {
            return Ok(Some(value));
        }

        Ok(None)
    }

    /// Get metadata map.
    pub fn get_metadata_map(&self, name: &str) -> Result<Option<&Map>, String> {
        if let Some(metadata) = self.metadata()?
            && let Some(value) = metadata.get(&name.into())
            && let Any::AnyMap(value) = value
        {
            return Ok(Some(value.to_map()));
        }

        Ok(None)
    }

    /// Read-only.
    pub fn is_read_only(&self) -> Result<bool, String> {
        let read_only =
            self.any.into_get("read_only").ok_or_else(|| format!("property {} is missing read_only", self.name))?;
        match read_only {
            Any::Boolean(read_only) => Ok(*read_only),
            _ => Err(format!("property {}: malformed \"read_only\", not a boolean", self.name)),
        }
    }
}
