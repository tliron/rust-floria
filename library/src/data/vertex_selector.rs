use super::{call::*, id::*, vertex_finder::*};

use {
    compris::{annotate::*, normal::*},
    kutil::cli::depict::*,
    std::io,
};

//
// VertexSelector
//

/// Vertex selector.
#[derive(Clone, Debug)]
pub enum VertexSelector {
    /// Vertex ID.
    VertexID(ID),

    /// Finder
    Finder(VertexFinder),
}

impl VertexSelector {
    /// Constructor.
    pub fn new_vertex(vertex_id: ID) -> Self {
        Self::VertexID(vertex_id)
    }

    /// Constructor.
    pub fn new_finder(finder: Call) -> Self {
        Self::Finder(VertexFinder::new(finder))
    }

    /// To Compris variant.
    pub fn to_variant<AnnotatedT>(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = Map::default();

        match self {
            Self::VertexID(id) => {
                map.into_insert("id", id.to_string());
            }
            Self::Finder(vertex_finder) => {
                map.into_insert("finder", vertex_finder.to_variant());
            }
        }

        map.into()
    }

    /// Select.
    #[cfg(feature = "plugins")]
    pub fn select<StoreT, ErrorRecipientT>(
        &self,
        source_vertex_id: &ID,
        edge_template_id: &ID,
        library: &mut super::super::plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<ID>, super::super::FloriaError>
    where
        StoreT: Clone + Send + super::super::Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        match self {
            Self::VertexID(id) => Ok(Some(id.clone())),
            Self::Finder(vertex_finder) => vertex_finder.find(source_vertex_id, edge_template_id, library, errors),
        }
    }
}

impl Depict for VertexSelector {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::VertexID(id) => id.depict(writer, context),
            Self::Finder(vertex_filter) => vertex_filter.depict(writer, context),
        }
    }
}
