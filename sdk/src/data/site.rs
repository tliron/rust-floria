use super::{
    super::{dispatch_bindings::*, host},
    entity::*,
};

use std::fmt;

impl Site {
    /// Vertex or edge.
    pub fn get_entity(&self) -> Result<Entity, String> {
        Ok(Entity::new(self.id.clone(), host::get_entity(&self.id)?))
    }

    /// Property.
    pub fn get_property(&self) -> Result<Option<Any>, String> {
        if let Some(property_name) = &self.property_name {
            let entity = self.get_entity()?;
            if let Some(property) = entity.get_property(property_name)? {
                let value = property.get_value()?;
                return Ok(Some(value.clone()));
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
