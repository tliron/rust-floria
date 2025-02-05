use super::super::super::data::*;

use kutil::std::collections::*;

//
// InMemoryStoreBackend
//

/// In-memory store backend.
#[derive(Default)]
pub struct InMemoryStoreBackend {
    pub(crate) next_id: FastConcurrentHashMap<Kind, u64>,
    pub(crate) classes: FastConcurrentHashMap<ID, Class>,
    pub(crate) vertex_templates: FastConcurrentHashMap<ID, VertexTemplate>,
    pub(crate) edge_templates: FastConcurrentHashMap<ID, EdgeTemplate>,
    pub(crate) vertexes: FastConcurrentHashMap<ID, Vertex>,
    pub(crate) edges: FastConcurrentHashMap<ID, Edge>,
}

impl InMemoryStoreBackend {
    /// Next ID.
    pub fn get_next_id(&self, kind: Kind) -> u64 {
        self.next_id.pin().update_or_insert(kind, |id| id + 1, 1).clone()
    }
}
