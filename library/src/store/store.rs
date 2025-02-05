use super::{super::data::*, errors::*};

use compris::{annotate::*, normal::*};

//
// Store
//

/// Thread-safe access to a Floria store.
///
/// Implementations should ensure that cloning is cheap and clones always refer to the same shared
/// state.
pub trait Store {
    /// Create ID.
    fn create_id(&self, id: &mut ID) -> Result<(), StoreError>;

    /// Get class.
    fn get_class(&self, id: &ID) -> Result<Option<Class>, StoreError>;

    /// Add class.
    fn add_class(&self, class: Class) -> Result<(), StoreError>;

    /// Get vertex template.
    fn get_vertex_template(&self, id: &ID) -> Result<Option<VertexTemplate>, StoreError>;

    /// Add vertex template.
    ///
    /// Checks to make sure we aren't creating infinite nesting.
    fn add_vertex_template(&self, vertex_template: VertexTemplate) -> Result<(), StoreError>;

    /// Get edge template.
    fn get_edge_template(&self, id: &ID) -> Result<Option<EdgeTemplate>, StoreError>;

    /// Add edge template.
    fn add_edge_template(&self, edge_template: EdgeTemplate) -> Result<(), StoreError>;

    /// Get vertex.
    fn get_vertex(&self, id: &ID) -> Result<Option<Vertex>, StoreError>;

    /// Get vertexes.
    fn get_vertexes(&self, directories: Option<Vec<Directory>>) -> Result<Vec<Vertex>, StoreError>;

    /// Add vertex.
    fn add_vertex(&self, vertex: Vertex) -> Result<(), StoreError>;

    /// Get edge.
    fn get_edge(&self, id: &ID) -> Result<Option<Edge>, StoreError>;

    /// Add edge.
    fn add_edge(&self, edge: Edge) -> Result<(), StoreError>;
}

//
// StoreUtilities
//

/// Utilities for [Store].
pub trait StoreUtilities {
    /// Get entity as [Variant].
    fn get_entity_as_variant<AnnotatedT>(&self, id: &ID) -> Result<Option<Variant<AnnotatedT>>, StoreError>
    where
        AnnotatedT: Annotated + Clone + Default;
}

impl<StoreT> StoreUtilities for StoreT
where
    StoreT: Store,
{
    fn get_entity_as_variant<AnnotatedT>(&self, id: &ID) -> Result<Option<Variant<AnnotatedT>>, StoreError>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let value_result = match id.kind {
            Kind::Class => self.get_class(&id)?.map(|class| Ok(class.to_variant())),
            Kind::VertexTemplate => {
                self.get_vertex_template(&id)?.map(|vertex_template| vertex_template.to_variant(false, self))
            }
            Kind::EdgeTemplate => {
                self.get_edge_template(&id)?.map(|edge_template| edge_template.to_variant(false, self))
            }
            Kind::Vertex => self.get_vertex(&id)?.map(|vertex| vertex.to_variant(false, self)),
            Kind::Edge => self.get_edge(&id)?.map(|edge| edge.to_variant(false, self)),
        };

        Ok(match value_result {
            Some(value_result) => Some(value_result?),
            None => None,
        })
    }
}
