use super::{
    super::{super::data::*, client::*, errors::*},
    store::*,
};

use {std::sync::*, tracing::info};

//
// InMemoryStoreClient
//

/// In-memory store client.
#[derive(Clone)]
pub struct InMemoryStoreClient {
    store: Arc<InMemoryStore>,
}

impl InMemoryStoreClient {
    /// Constructor.
    pub fn new(store: InMemoryStore) -> Self {
        Self { store: store.into() }
    }
}

impl StoreClient for InMemoryStoreClient {
    fn create_id(&self, id: &mut ID) -> Result<(), StoreError> {
        let next_id = self.store.get_next_id(id.kind.clone());
        id.id = next_id.to_string();
        info!("create_id: {}", id);
        Ok(())
    }

    fn get_group(&self, id: &ID) -> Result<Option<Group>, StoreError> {
        info!("get_group: {}", id);
        if id.kind != Kind::Group {
            return Err(StoreError::ID(format!("kind is not Group: {}", id.kind)));
        }
        Ok(self.store.groups.pin().get(id).map(|t| t.clone()))
    }

    fn add_group(&self, group: Group) -> Result<(), StoreError> {
        info!("add_group: {}", group.id);
        if group.id.kind != Kind::Group {
            return Err(StoreError::ID(format!("kind is not Group: {}", group.id.kind)));
        }
        self.store.groups.pin().insert(group.id.clone(), group);
        Ok(())
    }

    fn get_node_template(&self, id: &ID) -> Result<Option<NodeTemplate>, StoreError> {
        info!("get_node_template: {}", id);
        if id.kind != Kind::NodeTemplate {
            return Err(StoreError::ID(format!("kind is not NodeTemplate: {}", id.kind)));
        }
        Ok(self.store.node_templates.pin().get(id).map(|n| n.clone()))
    }

    fn add_node_template(&self, node_template: NodeTemplate) -> Result<(), StoreError> {
        info!("add_node_template: {}", node_template.template.id);
        if node_template.template.id.kind != Kind::NodeTemplate {
            return Err(StoreError::ID(format!("kind is not NodeTemplate: {}", node_template.template.id.kind)));
        }
        self.store.node_templates.pin().insert(node_template.template.id.clone(), node_template);
        Ok(())
    }

    fn get_relationship_template(&self, id: &ID) -> Result<Option<RelationshipTemplate>, StoreError> {
        info!("get_relationship_template: {}", id);
        if id.kind != Kind::RelationshipTemplate {
            return Err(StoreError::ID(format!("kind is not RelationshipTemplate: {}", id.kind)));
        }
        Ok(self.store.relationship_templates.pin().get(id).map(|r| r.clone()))
    }

    fn add_relationship_template(&self, relationship_template: RelationshipTemplate) -> Result<(), StoreError> {
        info!("add_relationship_template: {}", relationship_template.template.id);
        if relationship_template.template.id.kind != Kind::RelationshipTemplate {
            return Err(StoreError::ID(format!(
                "kind is not RelationshipTemplate: {}",
                relationship_template.template.id.kind
            )));
        }
        self.store
            .relationship_templates
            .pin()
            .insert(relationship_template.template.id.clone(), relationship_template);
        Ok(())
    }

    fn get_node(&self, id: &ID) -> Result<Option<Node>, StoreError> {
        info!("get_node: {}", id);
        if id.kind != Kind::Node {
            return Err(StoreError::ID(format!("kind is not Node: {}", id.kind)));
        }
        Ok(self.store.nodes.pin().get(id).map(|n| n.clone()))
    }

    fn get_nodes(&self, _namespaces: Option<Vec<Namespace>>) -> Result<Vec<Node>, StoreError> {
        Ok(self.store.nodes.pin().values().map(|n| n.clone()).collect())
    }

    fn add_node(&self, node: Node) -> Result<(), StoreError> {
        info!("add_node: {}", node.instance.id);
        if node.instance.id.kind != Kind::Node {
            return Err(StoreError::ID(format!("kind is not Node: {}", node.instance.id.kind)));
        }
        self.store.nodes.pin().insert(node.instance.id.clone(), node);
        Ok(())
    }

    fn get_relationship(&self, id: &ID) -> Result<Option<Relationship>, StoreError> {
        info!("get_relationship: {}", id);
        if id.kind != Kind::Relationship {
            return Err(StoreError::ID(format!("kind is not Relationship: {}", id.kind)));
        }
        Ok(self.store.relationships.pin().get(id).map(|r| r.clone()))
    }

    fn add_relationship(&self, relationship: Relationship) -> Result<(), StoreError> {
        info!("add_relationship: {}", relationship.instance.id);
        if relationship.instance.id.kind != Kind::Relationship {
            return Err(StoreError::ID(format!("kind is not Relationship: {}", relationship.instance.id.kind)));
        }
        self.store.relationships.pin().insert(relationship.instance.id.clone(), relationship);
        Ok(())
    }
}
