use super::super::super::data::*;

use {ahash::RandomState, papaya::*};

//
// InMemoryStore
//

/// In-memory store.
///
/// Based on Papaya [HashMap].
#[derive(Default)]
pub struct InMemoryStore {
    pub(crate) next_id: HashMap<Kind, u64, RandomState>,
    pub(crate) types: HashMap<ID, Type, RandomState>,
    pub(crate) node_templates: HashMap<ID, NodeTemplate, RandomState>,
    pub(crate) relationship_templates: HashMap<ID, RelationshipTemplate, RandomState>,
    pub(crate) nodes: HashMap<ID, Node, RandomState>,
    pub(crate) relationships: HashMap<ID, Relationship, RandomState>,
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
