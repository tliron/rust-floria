use super::super::super::data::*;

use kutil_std::collections::*;

//
// InMemoryStoreImplementation
//

/// In-memory store implementation.
#[derive(Default)]
pub struct InMemoryStoreImplementation<AnnotatedT> {
    pub(crate) next_id: FastConcurrentHashMap<Kind, u64>,
    pub(crate) groups: FastConcurrentHashMap<ID, Group<AnnotatedT>>,
    pub(crate) node_templates: FastConcurrentHashMap<ID, NodeTemplate<AnnotatedT>>,
    pub(crate) relationship_templates: FastConcurrentHashMap<ID, RelationshipTemplate<AnnotatedT>>,
    pub(crate) nodes: FastConcurrentHashMap<ID, Node<AnnotatedT>>,
    pub(crate) relationships: FastConcurrentHashMap<ID, Relationship<AnnotatedT>>,
}

impl<AnnotatedT> InMemoryStoreImplementation<AnnotatedT> {
    /// Constructor.
    pub fn new() -> Self
    where
        AnnotatedT: Default,
    {
        Self::default()
    }

    /// Next ID.
    pub fn get_next_id(&self, kind: Kind) -> u64 {
        self.next_id.pin().update_or_insert(kind, |id| id + 1, 1).clone()
    }
}
