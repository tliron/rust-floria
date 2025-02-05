use super::{
    super::{host, normal::*},
    id::*,
    kind::*,
};

//
// Entity
//

/// Entity.
#[derive(Clone)]
pub struct Entity {
    /// ID.
    pub id: ID,

    /// Value.
    pub value: Value,
}

impl Entity {
    /// Constructor.
    pub fn new(id: ID, value: Value) -> Self {
        Self { id, value }
    }

    /// Constructor.
    pub fn new_from(value: Value) -> Option<Self> {
        match ID::new_from(&value) {
            Some(id) => Some(Self::new(id, value)),
            None => None,
        }
    }

    /// Get.
    pub fn get(id: &ID) -> Result<Self, String> {
        match &id.kind {
            Kind::Node => Ok(Self::new(id.clone(), host::get_node(&id.id)?)),
            Kind::Relationship => Ok(Self::new(id.clone(), host::get_relationship(&id.id)?)),
            kind => Err(format!("unsupported kind: {}", kind)),
        }
    }

    /// Get a node's containing node.
    pub fn get_containing_node(&self) -> Result<Entity, String> {
        Self::get(&self.get_containing_node_id()?)
    }

    /// Get a node's containing node ID.
    pub fn get_containing_node_id(&self) -> Result<ID, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"containing_node_id".into()) {
                Some(containing_node_id) => match containing_node_id {
                    Value::Text(containing_node_id) => Ok(ID::parse(Kind::Node, containing_node_id)),
                    _ => Err("malformed containing_node_id".into()),
                },

                None => Err("malformed node".into()),
            },

            kind => Err(format!("not a node: {}", kind)),
        }
    }

    /// Get a node's node template ID.
    pub fn get_node_template_id(&self) -> Result<ID, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"node_template_id".into()) {
                Some(node_template_id) => match node_template_id {
                    Value::Text(node_template_id) => Ok(ID::parse(Kind::NodeTemplate, node_template_id)),
                    _ => Err("malformed node_template_id".into()),
                },

                None => Err("malformed node".into()),
            },

            kind => Err(format!("not a node: {}", kind)),
        }
    }

    /// Get a relationship's source node.
    pub fn get_source_node(&self) -> Result<Entity, String> {
        Self::get(&self.get_source_node_id()?)
    }

    /// Get a relationship's source node ID.
    pub fn get_source_node_id(&self) -> Result<ID, String> {
        match &self.id.kind {
            Kind::Relationship => match self.value.get(&"source_node_id".into()) {
                Some(source_node_id) => match source_node_id {
                    Value::Text(source_node_id) => Ok(ID::parse(Kind::Node, source_node_id)),
                    _ => Err("malformed source_node_id".into()),
                },

                None => Err("malformed relationship".into()),
            },

            kind => Err(format!("not a relationship: {}", kind)),
        }
    }
}
