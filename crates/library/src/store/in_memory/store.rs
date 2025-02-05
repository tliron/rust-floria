use super::super::super::data::*;

use kutil_std::collections::*;

//
// InMemoryStore
//

/// In-memory store.
///
/// Based on Papaya [HashMap].
#[derive(Default)]
pub struct InMemoryStore {
    pub(crate) next_id: FastConcurrentHashMap<Kind, u64>,
    pub(crate) groups: FastConcurrentHashMap<ID, Group>,
    pub(crate) node_templates: FastConcurrentHashMap<ID, NodeTemplate>,
    pub(crate) relationship_templates: FastConcurrentHashMap<ID, RelationshipTemplate>,
    pub(crate) nodes: FastConcurrentHashMap<ID, Node>,
    pub(crate) relationships: FastConcurrentHashMap<ID, Relationship>,
}

impl InMemoryStore {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Next ID.
    pub fn get_next_id(&self, kind: Kind) -> u64 {
        self.next_id.pin().update_or_insert(kind, |i| i + 1, 1).clone()
    }
}
