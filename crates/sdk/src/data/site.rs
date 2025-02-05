use super::{
    super::{dispatcher_bindings::Site, host},
    entity::*,
    id::*,
    kind::*,
};

use std::fmt;

impl Site {
    /// Kind.
    pub fn get_kind(&self) -> Result<Kind, String> {
        Kind::try_from(self.kind.as_str())
    }

    /// ID.
    pub fn get_id(&self) -> Result<ID, String> {
        Ok(ID::parse(self.get_kind()?, &self.id))
    }

    /// Node or relationship.
    pub fn get_entity(&self) -> Result<Entity, String> {
        let id = self.get_id()?;
        match id.kind {
            Kind::Node => self.get_as_node(),
            Kind::Relationship => self.get_as_relationship(),
            kind => Err(format!("unsupported entity kind: {}", kind)),
        }
    }

    /// Node.
    pub fn get_as_node(&self) -> Result<Entity, String> {
        Ok(Entity::new(self.get_id()?, host::get_node(&self.id)?))
    }

    /// Relationship.
    pub fn get_as_relationship(&self) -> Result<Entity, String> {
        Ok(Entity::new(self.get_id()?, host::get_relationship(&self.id)?))
    }
}

impl fmt::Display for Site {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.kind, self.id)
    }
}
