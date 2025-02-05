use super::{
    super::{super::data::*, errors::*, store::*, wrapper::*},
    backend::*,
};

use std::sync::*;

//
// InMemoryStore
//

/// In-memory store.
#[derive(Default, Clone)]
pub struct InMemoryStore {
    /// Backend.
    pub backend: Arc<InMemoryStoreBackend>,
}

impl InMemoryStore {
    /// Constructor.
    pub fn new(backend: Arc<InMemoryStoreBackend>) -> StoreWrapper<Self> {
        StoreWrapper::new(Self { backend })
    }
}

impl Store for InMemoryStore {
    fn create_id(&self, id: &mut ID) -> Result<(), StoreError> {
        let next_id = self.backend.get_next_id(id.kind.clone());
        id.id = next_id.to_string().into();
        Ok(())
    }

    fn get_class(&self, id: &ID) -> Result<Option<Class>, StoreError> {
        Ok(self.backend.classes.pin().get(id).cloned())
    }

    fn add_class(&self, class: Class) -> Result<(), StoreError> {
        self.backend.classes.pin().insert(class.id.clone(), class);
        Ok(())
    }

    fn get_vertex_template(&self, id: &ID) -> Result<Option<VertexTemplate>, StoreError> {
        Ok(self.backend.vertex_templates.pin().get(id).cloned())
    }

    fn add_vertex_template(&self, vertex_template: VertexTemplate) -> Result<(), StoreError> {
        self.backend.vertex_templates.pin().insert(vertex_template.template.id.clone(), vertex_template);
        Ok(())
    }

    fn get_edge_template(&self, id: &ID) -> Result<Option<EdgeTemplate>, StoreError> {
        Ok(self.backend.edge_templates.pin().get(id).cloned())
    }

    fn add_edge_template(&self, edge_template: EdgeTemplate) -> Result<(), StoreError> {
        self.backend.edge_templates.pin().insert(edge_template.template.id.clone(), edge_template);
        Ok(())
    }

    fn get_vertex(&self, id: &ID) -> Result<Option<Vertex>, StoreError> {
        Ok(self.backend.vertexes.pin().get(id).cloned())
    }

    fn get_vertexes(&self, _directories: Option<Vec<Directory>>) -> Result<Vec<Vertex>, StoreError> {
        Ok(self.backend.vertexes.pin().values().cloned().collect())
    }

    fn add_vertex(&self, vertex: Vertex) -> Result<(), StoreError> {
        self.backend.vertexes.pin().insert(vertex.instance.id.clone(), vertex);
        Ok(())
    }

    fn get_edge(&self, id: &ID) -> Result<Option<Edge>, StoreError> {
        Ok(self.backend.edges.pin().get(id).cloned())
    }

    fn add_edge(&self, edge: Edge) -> Result<(), StoreError> {
        self.backend.edges.pin().insert(edge.instance.id.clone(), edge);
        Ok(())
    }
}
