use super::{
    super::{dispatch_bindings::*, host},
    entity::*,
};

use std::fmt;

impl Site {
    /// Vertex or edge.
    pub fn entity(&self) -> Result<Entity, String> {
        Ok(Entity::new(self.id.clone(), host::get_entity(&self.id)?))
    }

    /// Property value.
    pub fn property_value(&self) -> Result<Option<Any>, String> {
        if let Some(property_name) = &self.property_name {
            let entity = self.entity()?;
            if let Some(property) = entity.get_property(property_name)?
                && let Some(value) = property.value()
            {
                return Ok(Some(value.clone()));
            }
        }

        Ok(None)
    }

    /// Get property metadata string.
    pub fn get_metadata_string(&self, name: &str) -> Result<Option<String>, String> {
        if let Some(property_name) = &self.property_name {
            let entity = self.entity()?;
            if let Some(property) = entity.get_property(property_name)?
                && let Some(metadata) = property.metadata()?
                && let Some(tosca_data) = metadata.get(&name.into())
                && let Any::Text(tosca_data) = tosca_data
            {
                return Ok(Some(tosca_data.clone()));
            }
        }

        Ok(None)
    }
}

impl fmt::Display for Site {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.property_name {
            Some(property_name) => write!(formatter, "{} {}", self.id, property_name),
            None => fmt::Display::fmt(&self.id, formatter),
        }
    }
}
