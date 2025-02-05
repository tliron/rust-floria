use super::{
    super::{dispatch_bindings::*, host},
    any::*,
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
        if let Some(property_name) = self.path.first() {
            let entity = self.entity()?;
            if let Some(property) = entity.get_property(property_name)?
                && let Some(value) = property.value()
            {
                // TODO: dive into property segments?
                return Ok(Some(value.clone()));
            }
        }

        Ok(None)
    }

    /// Get property metadata string.
    pub fn get_property_metadata_string(&self, name: &str) -> Result<Option<String>, String> {
        if let Some(property_name) = self.path.first() {
            let entity = self.entity()?;
            if let Some(property) = entity.get_property(property_name)? {
                return Ok(property.get_metadata_string(name)?.map(|value| value.into()));
            }
        }

        Ok(None)
    }

    /// Get property metadata map.
    pub fn get_property_metadata_map(&self, name: &str) -> Result<Option<Map>, String> {
        if let Some(property_name) = self.path.first() {
            let entity = self.entity()?;
            if let Some(property) = entity.get_property(property_name)? {
                return Ok(property.get_metadata_map(name)?.map(|value| value.clone()));
            }
        }

        Ok(None)
    }
}

impl fmt::Display for Site {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.id, formatter)?;
        for segment in &self.path {
            write!(formatter, ".{}", segment)?;
        }
        Ok(())
    }
}
