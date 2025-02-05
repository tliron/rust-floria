use super::{
    super::{dispatch_bindings::*, host},
    entity::*,
};

use std::fmt;

impl Site {
    /// Node or relationship.
    pub fn get_entity(&self) -> Result<Entity, String> {
        Ok(Entity::new(self.id.clone(), host::get_entity(&self.id)?))
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
