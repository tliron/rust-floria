use super::super::super::data::*;

use kutil::std::collections::*;

//
// InMemoryStoreImplementation
//

/// In-memory store implementation.
#[derive(Default)]
pub struct InMemoryStoreImplementation {
    pub(crate) next_id: FastConcurrentHashMap<Kind, u64>,
    pub(crate) classes: FastConcurrentHashMap<ID, Class>,
    pub(crate) vertex_templates: FastConcurrentHashMap<ID, VertexTemplate>,
    pub(crate) edge_templates: FastConcurrentHashMap<ID, EdgeTemplate>,
    pub(crate) vertexes: FastConcurrentHashMap<ID, Vertex>,
    pub(crate) edges: FastConcurrentHashMap<ID, Edge>,
}

impl InMemoryStoreImplementation {
    /// Next ID.
    pub fn get_next_id(&self, kind: Kind) -> u64 {
        self.next_id.pin().update_or_insert(kind, |id| id + 1, 1).clone()
    }
}
