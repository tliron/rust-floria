use super::{super::data::*, errors::*, store::*};

//
// StoreWrapper
//

/// [Store] wrapper.
#[derive(Clone, Debug)]
pub struct StoreWrapper<StoreT> {
    /// Inner.
    pub inner: StoreT,
}

impl<StoreT> StoreWrapper<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(inner: StoreT) -> Self {
        Self { inner }
    }
}

impl<StoreT> Store for StoreWrapper<StoreT>
where
    StoreT: Store,
{
    fn create_id(&self, id: &mut ID) -> Result<(), StoreError> {
        self.inner.create_id(id)?;
        tracing::debug!(id = id.to_string(), "create_id");
        Ok(())
    }

    fn get_class(&self, id: &ID) -> Result<Option<Class>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_class");
        if id.kind != Kind::Class {
            return Err(StoreError::ID(format!("kind is not Class: {}", id.kind)));
        }
        self.inner.get_class(id)
    }

    fn add_class(&self, class: Class) -> Result<(), StoreError> {
        tracing::debug!(id = class.id.to_string(), "add_class");
        if class.id.kind != Kind::Class {
            return Err(StoreError::ID(format!("kind is not Class: {}", class.id.kind)));
        }
        self.inner.add_class(class)
    }

    fn get_vertex_template(&self, id: &ID) -> Result<Option<VertexTemplate>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_vertex_template");
        if id.kind != Kind::VertexTemplate {
            return Err(StoreError::ID(format!("kind is not VertexTemplate: {}", id.kind)));
        }
        self.inner.get_vertex_template(id)
    }

    fn add_vertex_template(&self, vertex_template: VertexTemplate) -> Result<(), StoreError> {
        tracing::debug!(id = vertex_template.template.id.to_string(), "add_vertex_template");
        if vertex_template.template.id.kind != Kind::VertexTemplate {
            return Err(StoreError::ID(format!("kind is not VertexTemplate: {}", vertex_template.template.id.kind)));
        }
        self.inner.add_vertex_template(vertex_template)
    }

    fn get_edge_template(&self, id: &ID) -> Result<Option<EdgeTemplate>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_edge_template");
        if id.kind != Kind::EdgeTemplate {
            return Err(StoreError::ID(format!("kind is not EdgeTemplate: {}", id.kind)));
        }
        self.get_edge_template(id)
    }

    fn add_edge_template(&self, edge_template: EdgeTemplate) -> Result<(), StoreError> {
        tracing::debug!(id = edge_template.template.id.to_string(), "add_edge_template");
        if edge_template.template.id.kind != Kind::EdgeTemplate {
            return Err(StoreError::ID(format!("kind is not EdgeTemplate: {}", edge_template.template.id.kind)));
        }
        self.inner.add_edge_template(edge_template)
    }

    fn get_vertex(&self, id: &ID) -> Result<Option<Vertex>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_vertex");
        if id.kind != Kind::Vertex {
            return Err(StoreError::ID(format!("kind is not Vertex: {}", id.kind)));
        }
        self.inner.get_vertex(id)
    }

    fn get_vertexes(&self, directories: Option<Vec<Directory>>) -> Result<Vec<Vertex>, StoreError> {
        tracing::debug!("get_vertexes");
        self.inner.get_vertexes(directories)
    }

    fn add_vertex(&self, vertex: Vertex) -> Result<(), StoreError> {
        tracing::debug!(id = vertex.instance.id.to_string(), "add_vertex");
        if vertex.instance.id.kind != Kind::Vertex {
            return Err(StoreError::ID(format!("kind is not Vertex: {}", vertex.instance.id.kind)));
        }
        self.inner.add_vertex(vertex)
    }

    fn get_edge(&self, id: &ID) -> Result<Option<Edge>, StoreError> {
        tracing::debug!(id = id.to_string(), "get_edge");
        if id.kind != Kind::Edge {
            return Err(StoreError::ID(format!("kind is not Edge: {}", id.kind)));
        }
        self.inner.get_edge(id)
    }

    fn add_edge(&self, edge: Edge) -> Result<(), StoreError> {
        tracing::debug!(id = edge.instance.id.to_string(), "add_edge");
        if edge.instance.id.kind != Kind::Edge {
            return Err(StoreError::ID(format!("kind is not Edge: {}", edge.instance.id.kind)));
        }
        self.inner.add_edge(edge)
    }
}
