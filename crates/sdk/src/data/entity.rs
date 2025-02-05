use super::{
    super::{host, normal::*},
    id::*,
    kind::*,
};

//
// Entity
//

/// Entity.
#[derive(Clone, Debug)]
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

    /// Has type.
    pub fn has_type(&self, type_id: &ID) -> Result<bool, String> {
        match self.value.get(&"types".into()) {
            Some(types) => match types {
                Value::NestedList(type_ids) => {
                    for type_id_ in &type_ids.to_list().value {
                        match type_id_ {
                            Value::Text(type_id_) => {
                                if ID::parse(Kind::Type, type_id_) == *type_id {
                                    return Ok(true);
                                }
                            }

                            _ => {
                                return Err(format!("entity {} has malformed types: not a string", self.id));
                            }
                        }
                    }
                }

                _ => return Err(format!("entity {} has malformed types: not a list", self.id)),
            },

            None => {}
        }

        Ok(false)
    }

    /// Get a node's containing node.
    pub fn get_containing_node(&self) -> Result<Option<Entity>, String> {
        match self.get_containing_node_id()? {
            Some(node_id) => Ok(Some(Self::get(&node_id)?)),
            None => Ok(None),
        }
    }

    /// Get a node's containing node ID.
    pub fn get_containing_node_id(&self) -> Result<Option<ID>, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"containing_node_id".into()) {
                Some(containing_node_id) => match containing_node_id {
                    Value::Text(containing_node_id) => Ok(Some(ID::parse(Kind::Node, containing_node_id))),

                    _ => Err(format!("node {} has malformed containing_node_id: not a string", self.id)),
                },

                None => Ok(None),
            },

            kind => Err(format!("not a node: {}", kind)),
        }
    }

    /// Get an entity's origin template ID.
    pub fn get_origin_template_id(&self) -> Result<Option<ID>, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"origin_template_id".into()) {
                Some(origin_template_id) => match origin_template_id {
                    Value::Text(origin_template_id) => Ok(Some(ID::parse(Kind::NodeTemplate, origin_template_id))),

                    _ => Err(format!("node {} has malformed origin_template_id: not a string", self.id)),
                },

                None => Ok(None),
            },

            Kind::Relationship => match self.value.get(&"origin_template_id".into()) {
                Some(origin_template_id) => match origin_template_id {
                    Value::Text(origin_template_id) => {
                        Ok(Some(ID::parse(Kind::RelationshipTemplate, origin_template_id)))
                    }

                    _ => Err(format!("relationship {} has malformed origin_template_id: not a string", self.id)),
                },

                None => Ok(None),
            },

            kind => Err(format!("entity {} is not a node or a relationship: {}", self.id, kind)),
        }
    }

    /// Get a node's contained node IDs.
    pub fn get_contained_node_ids(&self) -> Result<Vec<ID>, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"contained_node_ids".into()) {
                Some(contained_node_ids) => match contained_node_ids {
                    Value::NestedList(contained_node_ids) => {
                        let contained_node_ids = contained_node_ids.to_list();

                        let mut contained_node_ids_ = Vec::with_capacity(contained_node_ids.value.len());
                        for contained_node_id in &contained_node_ids.value {
                            match contained_node_id {
                                Value::Text(contained_node_id) => {
                                    contained_node_ids_.push(ID::parse(Kind::Node, &contained_node_id));
                                }

                                _ => {
                                    return Err(format!(
                                        "node {} has malformed contained_node_ids: not a string",
                                        self.id
                                    ));
                                }
                            }
                        }

                        Ok(contained_node_ids_)
                    }

                    _ => Err(format!("node {} malformed contained_node_ids: not a list", self.id)),
                },

                None => Ok(Vec::new()),
            },

            kind => Err(format!("entity {} is not a node: {}", self.id, kind)),
        }
    }

    /// Get node's outgoing relationship IDs.
    pub fn get_outgoing_relationship_ids(&self) -> Result<Vec<ID>, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"outgoing_relationship_ids".into()) {
                Some(outgoing_relationship_ids) => match outgoing_relationship_ids {
                    Value::NestedList(outgoing_relationship_id) => {
                        let outgoing_relationship_ids = outgoing_relationship_id.to_list();

                        let mut outgoing_relationship_ids_ = Vec::with_capacity(outgoing_relationship_ids.value.len());
                        for outgoing_relationship_id in &outgoing_relationship_ids.value {
                            match outgoing_relationship_id {
                                Value::Text(outgoing_relationship_id) => {
                                    outgoing_relationship_ids_
                                        .push(ID::parse(Kind::Relationship, &outgoing_relationship_id));
                                }

                                _ => {
                                    return Err(format!(
                                        "node {} has malformed outgoing_relationship_ids: not a string",
                                        self.id
                                    ));
                                }
                            }
                        }

                        Ok(outgoing_relationship_ids_)
                    }

                    _ => Err(format!("node {} has malformed outgoing_relationship_ids: not a list", self.id)),
                },

                None => Ok(Vec::new()),
            },

            kind => Err(format!("entity {} is not a node: {}", self.id, kind)),
        }
    }

    /// Get node's incoming relationship IDs.
    pub fn get_incoming_relationship_ids(&self) -> Result<Vec<ID>, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"incoming_relationship_ids".into()) {
                Some(incoming_relationship_ids) => match incoming_relationship_ids {
                    Value::NestedList(incoming_relationship_id) => {
                        let incoming_relationship_ids = incoming_relationship_id.to_list();

                        let mut incoming_relationship_ids_ = Vec::with_capacity(incoming_relationship_ids.value.len());
                        for incoming_relationship_id in &incoming_relationship_ids.value {
                            match incoming_relationship_id {
                                Value::Text(incoming_relationship_id) => {
                                    incoming_relationship_ids_
                                        .push(ID::parse(Kind::Relationship, &incoming_relationship_id));
                                }

                                _ => {
                                    return Err(format!(
                                        "node {} has malformed incoming_relationship_ids: not a string",
                                        self.id
                                    ));
                                }
                            }
                        }

                        Ok(incoming_relationship_ids_)
                    }

                    _ => Err(format!("node {} has malformed incoming_relationship_ids: not a list", self.id)),
                },

                None => Ok(Vec::new()),
            },

            kind => Err(format!("entity {} is not a node: {}", self.id, kind)),
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
                    _ => Err(format!("relationship {} has malformed source_node_id: not a string", self.id)),
                },

                None => Err(format!("relationship {} is missing source_node_id", self.id)),
            },

            kind => Err(format!("entity {} is not a relationship: {}", self.id, kind)),
        }
    }

    /// Get a relationship's target node.
    pub fn get_target_node(&self) -> Result<Entity, String> {
        Self::get(&self.get_target_node_id()?)
    }

    /// Get a relationship's target node ID.
    pub fn get_target_node_id(&self) -> Result<ID, String> {
        match &self.id.kind {
            Kind::Relationship => match self.value.get(&"target_node_id".into()) {
                Some(target_node_id) => match target_node_id {
                    Value::Text(target_node_id) => Ok(ID::parse(Kind::Node, target_node_id)),
                    _ => Err(format!("relationship {} has malformed target_node_id: not a string", self.id)),
                },

                None => Err(format!("relationship {} is missing target_node_id", self.id)),
            },

            kind => Err(format!("entity {} is not a relationship: {}", self.id, kind)),
        }
    }

    /// Get a property.
    pub fn get_property(&self, property_name: &str) -> Result<Option<Property>, String> {
        match self.value.get(&"properties".into()) {
            Some(object) => Ok(object.get(&property_name.into()).map(|p| Property::new(property_name.into(), p))),
            None => Err(format!("entity {} is missing properties", self.id)),
        }
    }
}

//
// Property
//

/// Property.
#[derive(Clone)]
pub struct Property<'own> {
    /// Name.
    pub name: String,

    /// Value.
    pub value: &'own Value,
}

impl<'own> Property<'own> {
    /// Constructor.
    pub fn new(name: String, value: &'own Value) -> Self {
        Self { name, value }
    }

    /// Value.
    pub fn get_value(&self) -> Result<&Value, String> {
        match self.value.get(&"value".into()) {
            Some(value) => Ok(value),
            None => Err(format!("property {} is missing value", self.name)),
        }
    }

    /// Read-only.
    pub fn is_read_only(&self) -> Result<bool, String> {
        match self.value.get(&"read_only".into()) {
            Some(read_only) => match read_only {
                Value::Boolean(read_only) => Ok(*read_only),
                _ => Err(format!("property {}: malformed read_only, not a boolean", self.name)),
            },
            None => Err(format!("property {} is missing read_only", self.name)),
        }
    }
}
