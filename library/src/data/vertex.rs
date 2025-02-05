use super::{super::store::*, depict::*, directory::*, id::*, instance::*, kind::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{
        cli::depict::*,
        std::{immutable::*, iter::*},
    },
    std::io,
};

//
// Vertex
//

/// Vertex.
#[derive(Clone, Debug)]
pub struct Vertex {
    /// Instance.
    pub instance: Instance,

    /// Containing vertex ID.
    pub containing_vertex_id: Option<ID>,

    /// Contained vertex IDs.
    pub contained_vertex_ids: Vec<ID>,

    /// Outgoing edges.
    pub outgoing_edge_ids: Vec<ID>,

    /// Incoming edges.
    pub incoming_edge_ids: Vec<ID>,
}

impl Vertex {
    /// Constructor.
    pub fn new<StoreT>(directory: Directory, origin_template_id: ID, store: &StoreT) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(Kind::Vertex, directory);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, Some(origin_template_id)))
    }

    /// Constructor.
    pub fn new_for(directory: Directory, id: ByteString, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(Kind::Vertex, directory, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self {
            instance: Instance::new_with(id, origin_template_id),
            containing_vertex_id: None,
            contained_vertex_ids: Default::default(),
            outgoing_edge_ids: Default::default(),
            incoming_edge_ids: Default::default(),
        }
    }

    /// To Compris variant.
    pub fn to_variant<'own, StoreT, AnnotatedT>(
        &self,
        embedded: bool,
        store: &'own StoreT,
    ) -> Result<Variant<AnnotatedT>, StoreError>
    where
        AnnotatedT: Annotated + Clone + Default,
        StoreT: Store,
    {
        let mut map = Map::default();

        self.instance.to_variant(&mut map, embedded, store)?;

        if !embedded {
            if let Some(containing_vertex_id) = &self.containing_vertex_id {
                map.into_insert("containing_vertex_id", containing_vertex_id.to_string());
            }
        }

        if !self.contained_vertex_ids.is_empty() {
            if embedded {
                let mut contained_vertexes = List::new_with_capacity(self.contained_vertex_ids.len());
                for contained_vertex_id in &self.contained_vertex_ids {
                    match store.get_vertex(contained_vertex_id)? {
                        Some(vertex) => contained_vertexes.inner.push(vertex.to_variant(embedded, store)?),
                        None => {}
                    }
                }
                map.into_insert("contained_vertexes", contained_vertexes);
            } else {
                let contained_vertex_ids: List<_> =
                    self.contained_vertex_ids.iter().map(|id| id.to_string().into()).collect();
                map.into_insert("contained_vertex_ids", contained_vertex_ids);
            }
        }

        if !self.outgoing_edge_ids.is_empty() {
            if embedded {
                let mut outgoing_edges = List::new_with_capacity(self.outgoing_edge_ids.len());
                for outgoing_edge_id in &self.outgoing_edge_ids {
                    if let Some(edge) = store.get_edge(outgoing_edge_id)? {
                        outgoing_edges.inner.push(edge.to_variant(embedded, store)?);
                    }
                }
                map.into_insert("outgoing_edges", outgoing_edges);
            } else {
                let outgoing_edge_ids: List<_> =
                    self.outgoing_edge_ids.iter().map(|id| id.to_string().into()).collect();
                map.into_insert("outgoing_edge_ids", outgoing_edge_ids);
            }
        }

        if !embedded && !self.incoming_edge_ids.is_empty() {
            let incoming_edge_ids: List<_> = self.incoming_edge_ids.iter().map(|id| id.to_string().into()).collect();
            map.into_insert("incoming_edge_ids", incoming_edge_ids);
        }

        Ok(map.into())
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictVertex<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictVertex { vertex: self, store }
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        library: &mut super::super::plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), super::super::FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        if self.instance.update(library, errors)? {
            library.store.add_vertex(self.clone())?;
            if !self.instance.validate(library, errors)? {
                library.store.add_vertex(self.clone())?;
            }
        }

        for vertex_id in &self.contained_vertex_ids {
            if let Some(mut vertex) = library.store.get_vertex(vertex_id)? {
                vertex.update(library, errors)?;
            }
        }

        Ok(())
    }

    /// Instantiate edges.
    #[cfg(feature = "plugins")]
    pub fn instantiate_edges<StoreT, ErrorRecipientT>(
        &self,
        directory: &Directory,
        library: &mut super::super::plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), super::super::FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        use kutil::std::error::*;

        for contained_vertex_id in &self.contained_vertex_ids {
            match library.store.get_vertex(contained_vertex_id)? {
                Some(contained_vertex) => {
                    contained_vertex.instantiate_edges(directory, library, errors)?;
                }

                None => tracing::warn!("vertex not found: {}", contained_vertex_id),
            }
        }

        let mut vertex = self.clone();

        match &vertex.instance.origin_template_id {
            Some(origin_template_id) => match library.store.get_vertex_template(origin_template_id)? {
                Some(vertex_template) => {
                    for outgoing_edge_template_id in &vertex_template.outgoing_edge_template_ids {
                        match library.store.get_edge_template(outgoing_edge_template_id)? {
                            Some(outgoing_edge_template) => {
                                match outgoing_edge_template.target_selector.select(
                                    &vertex.instance.id,
                                    outgoing_edge_template_id,
                                    library,
                                    errors,
                                )? {
                                    Some(target_vertex_id) => {
                                        let outgoing_edge_id = outgoing_edge_template.instantiate(
                                            directory,
                                            vertex.instance.id.clone(),
                                            target_vertex_id,
                                            &library.store,
                                        )?;

                                        vertex.outgoing_edge_ids.push(outgoing_edge_id);
                                    }

                                    None => errors.give(super::super::FloriaError::Instantiation(
                                        "target vertex not found".into(),
                                    ))?,
                                }
                            }

                            None => {
                                tracing::warn!("edge template not found: {}", outgoing_edge_template_id)
                            }
                        }
                    }
                }

                None => tracing::warn!("vertex template not found: {}", origin_template_id),
            },

            None => {}
        }

        library.store.add_vertex(vertex)?;

        Ok(())
    }
}

//
// DepictVertex
//

/// Depict vertex.
pub struct DepictVertex<'own, StoreT>
where
    StoreT: Store,
{
    vertex: &'own Vertex,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictVertex<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "Vertex")?;
        depict_id("id", Some(&self.vertex.instance.id), false, writer, context)?;
        depict_id("origin_template_id", self.vertex.instance.origin_template_id.as_ref(), false, writer, context)?;
        depict_metadata(&self.vertex.instance.metadata, false, writer, context)?;
        depict_classes(&self.vertex.instance.class_ids, self.store, writer, context)?;
        depict_properties("properties", &self.vertex.instance.properties, self.store, false, writer, context)?;

        utils::depict_field("contained_vertexes", false, writer, context, |writer, context| -> io::Result<()> {
            if self.vertex.contained_vertex_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (vertex_id, last) in IterateWithLast::new(&self.vertex.contained_vertex_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_vertex(vertex_id).map_err(io::Error::other)? {
                        Some(vertex) => {
                            vertex
                                .to_depict(self.store)
                                .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            vertex_id.depict(writer, &context.child().with_separator(false))?;
                        }
                    }
                }
            }

            Ok(())
        })?;

        utils::depict_field("outgoing_edges", true, writer, context, |writer, context| -> io::Result<()> {
            if self.vertex.outgoing_edge_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (edge_id, last) in IterateWithLast::new(&self.vertex.outgoing_edge_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_edge(edge_id).map_err(io::Error::other)? {
                        Some(edge) => {
                            edge.to_depict(self.store)
                                .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            edge_id.depict(writer, &context.child().with_separator(false))?;
                        }
                    }
                }
            }

            Ok(())
        })
    }
}
