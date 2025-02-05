use super::{super::data::*, errors::*};

//
// Store
//

/// Thread-safe client access to a Floria store.
///
/// Cloning a client should be cheap. It is thus common to store shared state in
/// [Arc](std::sync::Arc), such that cloning a client would only require increasing reference
/// counters.
pub trait StoreClient: Clone + Send {
    /// Create ID.
    fn create_id(&self, id: &mut ID) -> Result<(), StoreError>;

    /// Get type.
    fn get_type(&self, id: &ID) -> Result<Option<Type>, StoreError>;

    /// Add type.
    fn add_type(&self, type_: Type) -> Result<(), StoreError>;

    /// Get node template.
    fn get_node_template(&self, id: &ID) -> Result<Option<NodeTemplate>, StoreError>;

    /// Add node template.
    ///
    /// Checks to make sure we aren't creating infinite nesting.
    fn add_node_template(&self, node_template: NodeTemplate) -> Result<(), StoreError>;

    /// Get relationship template.
    fn get_relationship_template(&self, id: &ID) -> Result<Option<RelationshipTemplate>, StoreError>;

    /// Add relationship template.
    fn add_relationship_template(&self, relationship_template: RelationshipTemplate) -> Result<(), StoreError>;

    /// Get node.
    fn get_node(&self, id: &ID) -> Result<Option<Node>, StoreError>;

    /// Get nodes.
    fn get_nodes(&self, namespaces: Option<Vec<Namespace>>) -> Result<Vec<Node>, StoreError>;

    /// Add node.
    fn add_node(&self, node: Node) -> Result<(), StoreError>;

    /// Get relationship.
    fn get_relationship(&self, id: &ID) -> Result<Option<Relationship>, StoreError>;

    /// Add relationship.
    fn add_relationship(&self, relationship: Relationship) -> Result<(), StoreError>;
}
