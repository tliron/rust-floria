use super::{super::data::*, errors::*};

use compris::normal::*;

//
// Store
//

/// Thread-safe access to a Floria store.
///
/// Cloning an implementation is commonly done and should be cheap. One way to achieve this is to
/// make use of [Arc](std::sync::Arc) for referencing shared state.
pub trait Store<AnnotatedT>: Clone + Send {
    /// Create ID.
    fn create_id(&self, id: &mut ID) -> Result<(), StoreError>;

    /// Get group.
    fn get_group(&self, id: &ID) -> Result<Option<Group<AnnotatedT>>, StoreError>;

    /// Add group.
    fn add_group(&self, group: Group<AnnotatedT>) -> Result<(), StoreError>;

    /// Get node template.
    fn get_node_template(&self, id: &ID) -> Result<Option<NodeTemplate<AnnotatedT>>, StoreError>;

    /// Add node template.
    ///
    /// Checks to make sure we aren't creating infinite nesting.
    fn add_node_template(&self, node_template: NodeTemplate<AnnotatedT>) -> Result<(), StoreError>;

    /// Get relationship template.
    fn get_relationship_template(&self, id: &ID) -> Result<Option<RelationshipTemplate<AnnotatedT>>, StoreError>;

    /// Add relationship template.
    fn add_relationship_template(
        &self,
        relationship_template: RelationshipTemplate<AnnotatedT>,
    ) -> Result<(), StoreError>;

    /// Get node.
    fn get_node(&self, id: &ID) -> Result<Option<Node<AnnotatedT>>, StoreError>;

    /// Get nodes.
    fn get_nodes(&self, namespaces: Option<Vec<Namespace>>) -> Result<Vec<Node<AnnotatedT>>, StoreError>;

    /// Add node.
    fn add_node(&self, node: Node<AnnotatedT>) -> Result<(), StoreError>;

    /// Get relationship.
    fn get_relationship(&self, id: &ID) -> Result<Option<Relationship<AnnotatedT>>, StoreError>;

    /// Add relationship.
    fn add_relationship(&self, relationship: Relationship<AnnotatedT>) -> Result<(), StoreError>;

    /// Get entity as value.
    fn get_entity_as_value(&self, id: &ID) -> Result<Option<Variant<AnnotatedT>>, StoreError>
    where
        AnnotatedT: Clone + Default,
    {
        let value_result = match id.kind {
            Kind::Group => self.get_group(&id)?.map(|group| Ok(group.to_variant())),
            Kind::NodeTemplate => {
                self.get_node_template(&id)?.map(|node_template| node_template.to_variant(false, self))
            }
            Kind::RelationshipTemplate => self
                .get_relationship_template(&id)?
                .map(|relationship_template| relationship_template.to_variant(false, self)),
            Kind::Node => self.get_node(&id)?.map(|node| node.to_variant(false, self)),
            Kind::Relationship => self.get_relationship(&id)?.map(|relationship| relationship.to_variant(false, self)),
        };

        Ok(match value_result {
            Some(value_result) => Some(value_result?),
            None => None,
        })
    }
}
