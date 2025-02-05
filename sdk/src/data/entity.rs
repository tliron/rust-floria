use super::{
    super::{dispatch_bindings::*, host},
    property::*,
};

//
// Entity
//

/// Entity.
#[derive(Clone, Debug)]
pub struct Entity {
    /// ID.
    pub id: Id,

    /// Any.
    pub any: Any,
}

impl Entity {
    /// Constructor.
    pub fn new(id: Id, any: Any) -> Self {
        Self { id, any }
    }

    /// Constructor.
    pub fn new_from(any: Any) -> Option<Self> {
        Id::new_from(&any).map(|id| Self::new(id, any))
    }

    /// Get.
    pub fn get(id: &Id) -> Result<Self, String> {
        Ok(Self::new(id.clone(), host::get_entity(id)?))
    }

    /// Is in class.
    pub fn is_in_class(&self, class_id: &Id) -> Result<bool, String> {
        match self.any.into_get("class_ids") {
            Some(class_ids) => match class_ids {
                Any::AnyList(class_ids) => {
                    for class_id_ in &class_ids.to_list().inner {
                        match class_id_ {
                            Any::Text(class_id_) => {
                                if Id::parse(Kind::Class, class_id_) == *class_id {
                                    return Ok(true);
                                }
                            }

                            _ => {
                                return Err(format!("entity {} has malformed class_ids: not a string", self.id));
                            }
                        }
                    }
                }

                _ => return Err(format!("entity {} has malformed class_ids: not a list", self.id)),
            },

            None => {}
        }

        Ok(false)
    }

    /// Get a vertex's containing vertex.
    pub fn get_containing_vertex(&self) -> Result<Option<Entity>, String> {
        Ok(match self.get_containing_vertex_id()? {
            Some(vertex_id) => Some(Self::get(&vertex_id)?),
            None => None,
        })
    }

    /// Get a vertex's containing vertex ID.
    pub fn get_containing_vertex_id(&self) -> Result<Option<Id>, String> {
        match &self.id.kind {
            Kind::Vertex => match self.any.into_get("containing_vertex_id") {
                Some(containing_vertex_id) => match containing_vertex_id {
                    Any::Text(containing_vertex_id) => Ok(Some(Id::parse(Kind::Vertex, containing_vertex_id))),

                    _ => Err(format!("vertex {} has malformed containing_vertex_id: not a string", self.id)),
                },

                None => Ok(None),
            },

            kind => Err(format!("not a vertex: {}", kind)),
        }
    }

    /// Get an entity's origin template ID.
    pub fn get_origin_template_id(&self) -> Result<Option<Id>, String> {
        match &self.id.kind {
            Kind::Vertex => match self.any.into_get("origin_template_id") {
                Some(origin_template_id) => match origin_template_id {
                    Any::Text(origin_template_id) => Ok(Some(Id::parse(Kind::VertexTemplate, origin_template_id))),

                    _ => Err(format!("vertex {} has malformed origin_template_id: not a string", self.id)),
                },

                None => Ok(None),
            },

            Kind::Edge => match self.any.into_get("origin_template_id") {
                Some(origin_template_id) => match origin_template_id {
                    Any::Text(origin_template_id) => Ok(Some(Id::parse(Kind::EdgeTemplate, origin_template_id))),

                    _ => Err(format!("edge {} has malformed origin_template_id: not a string", self.id)),
                },

                None => Ok(None),
            },

            kind => Err(format!("entity {} is not a vertex or an edge: {}", self.id, kind)),
        }
    }

    /// Get a vertex's contained vertex IDs.
    pub fn get_contained_vertex_ids(&self) -> Result<Vec<Id>, String> {
        match &self.id.kind {
            Kind::Vertex => match self.any.into_get("contained_vertex_ids") {
                Some(contained_vertex_ids) => match contained_vertex_ids {
                    Any::AnyList(contained_vertex_ids) => {
                        let contained_vertex_ids = contained_vertex_ids.to_list();

                        let mut contained_vertex_ids_ = Vec::with_capacity(contained_vertex_ids.inner.len());
                        for contained_vertex_id in &contained_vertex_ids.inner {
                            match contained_vertex_id {
                                Any::Text(contained_vertex_id) => {
                                    contained_vertex_ids_.push(Id::parse(Kind::Vertex, &contained_vertex_id));
                                }

                                _ => {
                                    return Err(format!(
                                        "vertex {} has malformed contained_vertex_ids: not a string",
                                        self.id
                                    ));
                                }
                            }
                        }

                        Ok(contained_vertex_ids_)
                    }

                    _ => Err(format!("vertex {} malformed contained_vertex_ids: not a list", self.id)),
                },

                None => Ok(Default::default()),
            },

            kind => Err(format!("entity {} is not a vertex: {}", self.id, kind)),
        }
    }

    /// Get vertex's outgoing edge IDs.
    pub fn get_outgoing_edge_ids(&self) -> Result<Vec<Id>, String> {
        match &self.id.kind {
            Kind::Vertex => match self.any.into_get("outgoing_edge_ids") {
                Some(outgoing_edge_ids) => match outgoing_edge_ids {
                    Any::AnyList(outgoing_edge_id) => {
                        let outgoing_edge_ids = outgoing_edge_id.to_list();

                        let mut outgoing_edge_ids_ = Vec::with_capacity(outgoing_edge_ids.inner.len());
                        for outgoing_edge_id in &outgoing_edge_ids.inner {
                            match outgoing_edge_id {
                                Any::Text(outgoing_edge_id) => {
                                    outgoing_edge_ids_.push(Id::parse(Kind::Edge, &outgoing_edge_id));
                                }

                                _ => {
                                    return Err(format!(
                                        "vertex {} has malformed outgoing_edge_ids: not a string",
                                        self.id
                                    ));
                                }
                            }
                        }

                        Ok(outgoing_edge_ids_)
                    }

                    _ => Err(format!("vertex {} has malformed outgoing_edge_ids: not a list", self.id)),
                },

                None => Ok(Default::default()),
            },

            kind => Err(format!("entity {} is not a vertex: {}", self.id, kind)),
        }
    }

    /// Get vertex's incoming edge IDs.
    pub fn get_incoming_edge_ids(&self) -> Result<Vec<Id>, String> {
        match &self.id.kind {
            Kind::Vertex => match self.any.into_get("incoming_edge_ids") {
                Some(incoming_edge_ids) => match incoming_edge_ids {
                    Any::AnyList(incoming_edge_id) => {
                        let incoming_edge_ids = incoming_edge_id.to_list();

                        let mut incoming_edge_ids_ = Vec::with_capacity(incoming_edge_ids.inner.len());
                        for incoming_edge_id in &incoming_edge_ids.inner {
                            match incoming_edge_id {
                                Any::Text(incoming_edge_id) => {
                                    incoming_edge_ids_.push(Id::parse(Kind::Edge, &incoming_edge_id));
                                }

                                _ => {
                                    return Err(format!(
                                        "vertex {} has malformed incoming_edge_ids: not a string",
                                        self.id
                                    ));
                                }
                            }
                        }

                        Ok(incoming_edge_ids_)
                    }

                    _ => Err(format!("vertex {} has malformed incoming_edge_ids: not a list", self.id)),
                },

                None => Ok(Default::default()),
            },

            kind => Err(format!("entity {} is not a vertex: {}", self.id, kind)),
        }
    }

    /// Get a edge's source vertex.
    pub fn get_source_vertex(&self) -> Result<Entity, String> {
        Self::get(&self.get_source_vertex_id()?)
    }

    /// Get a edge's source vertex ID.
    pub fn get_source_vertex_id(&self) -> Result<Id, String> {
        match &self.id.kind {
            Kind::Edge => match self.any.into_get("source_vertex_id") {
                Some(source_vertex_id) => match source_vertex_id {
                    Any::Text(source_vertex_id) => Ok(Id::parse(Kind::Vertex, source_vertex_id)),
                    _ => Err(format!("edge {} has malformed source_vertex_id: not a string", self.id)),
                },

                None => Err(format!("edge {} is missing source_vertex_id", self.id)),
            },

            kind => Err(format!("entity {} is not an edge: {}", self.id, kind)),
        }
    }

    /// Get a edge's target vertex.
    pub fn get_target_vertex(&self) -> Result<Entity, String> {
        Self::get(&self.get_target_vertex_id()?)
    }

    /// Get a edge's target vertex ID.
    pub fn get_target_vertex_id(&self) -> Result<Id, String> {
        match &self.id.kind {
            Kind::Edge => match self.any.into_get("target_vertex_id") {
                Some(target_vertex_id) => match target_vertex_id {
                    Any::Text(target_vertex_id) => Ok(Id::parse(Kind::Vertex, target_vertex_id)),
                    _ => Err(format!("edge {} has malformed target_vertex_id: not a string", self.id)),
                },

                None => Err(format!("edge {} is missing target_vertex_id", self.id)),
            },

            kind => Err(format!("entity {} is not an edge: {}", self.id, kind)),
        }
    }

    /// Get a property.
    pub fn get_property(&self, property_name: &str) -> Result<Option<Property<'_>>, String> {
        let object =
            self.any.into_get("properties").ok_or_else(|| format!("entity {} is missing properties", self.id))?;
        Ok(object.get(&property_name.into()).map(|property| Property::new(property_name.into(), property)))
    }
}
