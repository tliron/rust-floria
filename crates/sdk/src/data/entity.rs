use super::super::{dispatch_bindings::*, host};

//
// Entity
//

/// Entity.
#[derive(Clone, Debug)]
pub struct Entity {
    /// ID.
    pub id: Id,

    /// Value.
    pub value: Value,
}

impl Entity {
    /// Constructor.
    pub fn new(id: Id, value: Value) -> Self {
        Self { id, value }
    }

    /// Constructor.
    pub fn new_from(value: Value) -> Option<Self> {
        Id::new_from(&value).map(|id| Self::new(id, value))
    }

    /// Get.
    pub fn get(id: &Id) -> Result<Self, String> {
        Ok(Self::new(id.clone(), host::get_entity(id)?))
    }

    /// Is in group.
    pub fn is_in_group(&self, group_id: &Id) -> Result<bool, String> {
        match self.value.get(&"group_ids".into()) {
            Some(group_ids) => match group_ids {
                Value::NestedList(group_ids) => {
                    for group_id_ in &group_ids.to_list().value {
                        match group_id_ {
                            Value::Text(group_id_) => {
                                if Id::parse(Kind::Group, group_id_) == *group_id {
                                    return Ok(true);
                                }
                            }

                            _ => {
                                return Err(format!("entity {} has malformed group_ids: not a string", self.id));
                            }
                        }
                    }
                }

                _ => return Err(format!("entity {} has malformed group_ids: not a list", self.id)),
            },

            None => {}
        }

        Ok(false)
    }

    /// Get a node's containing node.
    pub fn get_containing_node(&self) -> Result<Option<Entity>, String> {
        Ok(match self.get_containing_node_id()? {
            Some(node_id) => Some(Self::get(&node_id)?),
            None => None,
        })
    }

    /// Get a node's containing node ID.
    pub fn get_containing_node_id(&self) -> Result<Option<Id>, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"containing_node_id".into()) {
                Some(containing_node_id) => match containing_node_id {
                    Value::Text(containing_node_id) => Ok(Some(Id::parse(Kind::Node, containing_node_id))),

                    _ => Err(format!("node {} has malformed containing_node_id: not a string", self.id)),
                },

                None => Ok(None),
            },

            kind => Err(format!("not a node: {}", kind)),
        }
    }

    /// Get an entity's origin template ID.
    pub fn get_origin_template_id(&self) -> Result<Option<Id>, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"origin_template_id".into()) {
                Some(origin_template_id) => match origin_template_id {
                    Value::Text(origin_template_id) => Ok(Some(Id::parse(Kind::NodeTemplate, origin_template_id))),

                    _ => Err(format!("node {} has malformed origin_template_id: not a string", self.id)),
                },

                None => Ok(None),
            },

            Kind::Relationship => match self.value.get(&"origin_template_id".into()) {
                Some(origin_template_id) => match origin_template_id {
                    Value::Text(origin_template_id) => {
                        Ok(Some(Id::parse(Kind::RelationshipTemplate, origin_template_id)))
                    }

                    _ => Err(format!("relationship {} has malformed origin_template_id: not a string", self.id)),
                },

                None => Ok(None),
            },

            kind => Err(format!("entity {} is not a node or a relationship: {}", self.id, kind)),
        }
    }

    /// Get a node's contained node IDs.
    pub fn get_contained_node_ids(&self) -> Result<Vec<Id>, String> {
        match &self.id.kind {
            Kind::Node => match self.value.get(&"contained_node_ids".into()) {
                Some(contained_node_ids) => match contained_node_ids {
                    Value::NestedList(contained_node_ids) => {
                        let contained_node_ids = contained_node_ids.to_list();

                        let mut contained_node_ids_ = Vec::with_capacity(contained_node_ids.value.len());
                        for contained_node_id in &contained_node_ids.value {
                            match contained_node_id {
                                Value::Text(contained_node_id) => {
                                    contained_node_ids_.push(Id::parse(Kind::Node, &contained_node_id));
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
    pub fn get_outgoing_relationship_ids(&self) -> Result<Vec<Id>, String> {
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
                                        .push(Id::parse(Kind::Relationship, &outgoing_relationship_id));
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
    pub fn get_incoming_relationship_ids(&self) -> Result<Vec<Id>, String> {
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
                                        .push(Id::parse(Kind::Relationship, &incoming_relationship_id));
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
    pub fn get_source_node_id(&self) -> Result<Id, String> {
        match &self.id.kind {
            Kind::Relationship => match self.value.get(&"source_node_id".into()) {
                Some(source_node_id) => match source_node_id {
                    Value::Text(source_node_id) => Ok(Id::parse(Kind::Node, source_node_id)),
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
    pub fn get_target_node_id(&self) -> Result<Id, String> {
        match &self.id.kind {
            Kind::Relationship => match self.value.get(&"target_node_id".into()) {
                Some(target_node_id) => match target_node_id {
                    Value::Text(target_node_id) => Ok(Id::parse(Kind::Node, target_node_id)),
                    _ => Err(format!("relationship {} has malformed target_node_id: not a string", self.id)),
                },

                None => Err(format!("relationship {} is missing target_node_id", self.id)),
            },

            kind => Err(format!("entity {} is not a relationship: {}", self.id, kind)),
        }
    }

    /// Get a property.
    pub fn get_property(&self, property_name: &str) -> Result<Option<Property>, String> {
        let object =
            self.value.get(&"properties".into()).ok_or_else(|| format!("entity {} is missing properties", self.id))?;
        Ok(object.get(&property_name.into()).map(|property| Property::new(property_name.into(), property)))
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
        self.value.get(&"value".into()).ok_or_else(|| format!("property {} is missing value", self.name))
    }

    /// Read-only.
    pub fn is_read_only(&self) -> Result<bool, String> {
        let read_only = self
            .value
            .get(&"read_only".into())
            .ok_or_else(|| format!("property {} is missing read_only", self.name))?;
        match read_only {
            Value::Boolean(read_only) => Ok(*read_only),
            _ => Err(format!("property {}: malformed read_only, not a boolean", self.name)),
        }
    }
}
