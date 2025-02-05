use super::super::super::data::*;

use kutil::std::collections::*;

//
// InMemoryStoreImplementation
//

/// In-memory store implementation.
#[derive(Default)]
pub struct InMemoryStoreImplementation {
    pub(crate) next_id: FastConcurrentHashMap<Kind, u64>,
    pub(crate) groups: FastConcurrentHashMap<ID, Group>,
    pub(crate) node_templates: FastConcurrentHashMap<ID, NodeTemplate>,
    pub(crate) relationship_templates: FastConcurrentHashMap<ID, RelationshipTemplate>,
    pub(crate) nodes: FastConcurrentHashMap<ID, Node>,
    pub(crate) relationships: FastConcurrentHashMap<ID, Relationship>,
}

impl InMemoryStoreImplementation {
    /// Next ID.
    pub fn get_next_id(&self, kind: Kind) -> u64 {
        self.next_id.pin().update_or_insert(kind, |id| id + 1, 1).clone()
    }
}
