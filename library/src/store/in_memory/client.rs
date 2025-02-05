use super::{
    super::{super::data::*, errors::*, store::*},
    implementation::*,
};

use std::sync::*;

//
// InMemoryStoreClient
//

/// In-memory store client.
#[derive(Clone)]
pub struct InMemoryStoreClient {
    /// Implementation.
    pub implementation: Arc<InMemoryStoreImplementation>,
}

impl InMemoryStoreClient {
    /// Constructor.
    pub fn new(store: Arc<InMemoryStoreImplementation>) -> Self {
        Self { implementation: store }
    }
}

impl Store for InMemoryStoreClient {
    fn create_id(&self, id: &mut ID) -> Result<(), StoreError> {
        let next_id = self.implementation.get_next_id(id.kind.clone());
        id.id = next_id.to_string().into();
        tracing::debug!(id = id.to_string(), "create_id");
        Ok(())
    }

    fn get_group(&self, id: &ID) -> Result<Option<Group>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_group");
        if id.kind != Kind::Group {
            return Err(StoreError::ID(format!("kind is not Group: {}", id.kind)));
        }
        Ok(self.implementation.groups.pin().get(id).cloned())
    }

    fn add_group(&self, group: Group) -> Result<(), StoreError> {
        tracing::debug!(id = group.id.to_string(), "add_group");
        if group.id.kind != Kind::Group {
            return Err(StoreError::ID(format!("kind is not Group: {}", group.id.kind)));
        }
        self.implementation.groups.pin().insert(group.id.clone(), group);
        Ok(())
    }

    fn get_node_template(&self, id: &ID) -> Result<Option<NodeTemplate>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_node_template");
        if id.kind != Kind::NodeTemplate {
            return Err(StoreError::ID(format!("kind is not NodeTemplate: {}", id.kind)));
        }
        Ok(self.implementation.node_templates.pin().get(id).cloned())
    }

    fn add_node_template(&self, node_template: NodeTemplate) -> Result<(), StoreError> {
        tracing::debug!(id = node_template.template.id.to_string(), "add_node_template");
        if node_template.template.id.kind != Kind::NodeTemplate {
            return Err(StoreError::ID(format!("kind is not NodeTemplate: {}", node_template.template.id.kind)));
        }
        self.implementation.node_templates.pin().insert(node_template.template.id.clone(), node_template);
        Ok(())
    }

    fn get_relationship_template(&self, id: &ID) -> Result<Option<RelationshipTemplate>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_relationship_template");
        if id.kind != Kind::RelationshipTemplate {
            return Err(StoreError::ID(format!("kind is not RelationshipTemplate: {}", id.kind)));
        }
        Ok(self.implementation.relationship_templates.pin().get(id).cloned())
    }

    fn add_relationship_template(&self, relationship_template: RelationshipTemplate) -> Result<(), StoreError> {
        tracing::debug!(id = relationship_template.template.id.to_string(), "add_relationship_template");
        if relationship_template.template.id.kind != Kind::RelationshipTemplate {
            return Err(StoreError::ID(format!(
                "kind is not RelationshipTemplate: {}",
                relationship_template.template.id.kind
            )));
        }
        self.implementation
            .relationship_templates
            .pin()
            .insert(relationship_template.template.id.clone(), relationship_template);
        Ok(())
    }

    fn get_node(&self, id: &ID) -> Result<Option<Node>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_node");
        if id.kind != Kind::Node {
            return Err(StoreError::ID(format!("kind is not Node: {}", id.kind)));
        }
        Ok(self.implementation.nodes.pin().get(id).cloned())
    }

    fn get_nodes(&self, _prefixes: Option<Vec<Prefix>>) -> Result<Vec<Node>, StoreError> {
        Ok(self.implementation.nodes.pin().values().cloned().collect())
    }

    fn add_node(&self, node: Node) -> Result<(), StoreError> {
        tracing::debug!(id = node.instance.id.to_string(), "add_node");
        if node.instance.id.kind != Kind::Node {
            return Err(StoreError::ID(format!("kind is not Node: {}", node.instance.id.kind)));
        }
        self.implementation.nodes.pin().insert(node.instance.id.clone(), node);
        Ok(())
    }

    fn get_relationship(&self, id: &ID) -> Result<Option<Relationship>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_relationship");
        if id.kind != Kind::Relationship {
            return Err(StoreError::ID(format!("kind is not Relationship: {}", id.kind)));
        }
        Ok(self.implementation.relationships.pin().get(id).cloned())
    }

    fn add_relationship(&self, relationship: Relationship) -> Result<(), StoreError> {
        tracing::debug!(id = relationship.instance.id.to_string(), "add_relationship");
        if relationship.instance.id.kind != Kind::Relationship {
            return Err(StoreError::ID(format!("kind is not Relationship: {}", relationship.instance.id.kind)));
        }
        self.implementation.relationships.pin().insert(relationship.instance.id.clone(), relationship);
        Ok(())
    }
}
