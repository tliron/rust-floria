use super::{super::store::*, depict::*, directory::*, id::*, kind::*, template::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{
        cli::depict::*,
        std::{immutable::*, iter::*},
    },
    std::io,
};

//
// VertexTemplate
//

/// Vertex template.
#[derive(Clone, Debug)]
pub struct VertexTemplate {
    /// Template.
    pub template: Template,

    /// Containing vertex template ID.
    pub containing_vertex_template_id: Option<ID>,

    /// Contained vertex template IDs.
    pub contained_vertex_template_ids: Vec<ID>,

    /// Outgoing edge template IDs.
    pub outgoing_edge_template_ids: Vec<ID>,
}

impl VertexTemplate {
    /// Constructor.
    pub fn new<StoreT>(directory: Directory, store: &StoreT) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(Kind::VertexTemplate, directory);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, None))
    }

    /// Constructor.
    pub fn new_for(directory: Directory, id: ByteString, containing_vertex_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(Kind::VertexTemplate, directory, id), containing_vertex_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, containing_vertex_template_id: Option<ID>) -> Self {
        Self {
            template: Template::new(id),
            containing_vertex_template_id,
            contained_vertex_template_ids: Default::default(),
            outgoing_edge_template_ids: Default::default(),
        }
    }

    /// Instantiate.
    #[cfg(feature = "plugins")]
    pub fn instantiate<StoreT, ErrorRecipientT>(
        &self,
        directory: &Directory,
        containing_vertex_id: Option<ID>,
        library: &mut super::super::plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<super::Vertex, super::super::FloriaError>
    where
        StoreT: Clone + Send + Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        let vertex_id = self.instantiate_vertexes(directory, containing_vertex_id, library, errors)?;
        let mut vertex =
            library.store.get_vertex(&vertex_id)?.ok_or_else(|| super::super::StoreError::ID(vertex_id.to_string()))?;

        vertex.update(library, errors)?;

        vertex.instantiate_edges(directory, library, errors)?;

        Ok(vertex)
    }

    /// Instantiate vertexes.
    #[cfg(feature = "plugins")]
    pub fn instantiate_vertexes<StoreT, ErrorRecipientT>(
        &self,
        directory: &Directory,
        containing_vertex_id: Option<ID>,
        library: &mut super::super::plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<ID, super::super::FloriaError>
    where
        StoreT: Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        let mut vertex = super::vertex::Vertex {
            instance: self.template.instantiate(Kind::Vertex, directory, &library.store)?,
            containing_vertex_id: containing_vertex_id,
            contained_vertex_ids: Vec::with_capacity(self.contained_vertex_template_ids.len()),
            outgoing_edge_ids: Default::default(),
            incoming_edge_ids: Default::default(),
        };

        let vertex_id = vertex.instance.id.clone();

        for contained_vertex_template_id in &self.contained_vertex_template_ids {
            match library.store.get_vertex_template(contained_vertex_template_id)? {
                Some(contained_vertex_template) => {
                    let contained_vertex_id = contained_vertex_template.instantiate_vertexes(
                        directory,
                        Some(vertex_id.clone()),
                        library,
                        errors,
                    )?;
                    vertex.contained_vertex_ids.push(contained_vertex_id);
                }

                None => tracing::warn!("vertex template not found: {}", contained_vertex_template_id),
            }
        }

        library.store.add_vertex(vertex)?;

        Ok(vertex_id)
    }

    /// To Compris variant.
    pub fn to_variant<'own, StoreT, AnnotatedT>(
        &self,
        debug: bool,
        store: &'own StoreT,
    ) -> Result<Variant<AnnotatedT>, StoreError>
    where
        StoreT: Store,
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = Map::default();

        self.template.to_variant(&mut map, debug, store)?;

        if !debug {
            if let Some(containing_vertex_template_id) = &self.containing_vertex_template_id {
                map.into_insert("containing_vertex_template_id", containing_vertex_template_id.id.clone());
            }
        }

        if !self.contained_vertex_template_ids.is_empty() {
            if debug {
                let mut contained_vertex_templates = List::new_with_capacity(self.contained_vertex_template_ids.len());
                for contained_vertex_template_id in &self.contained_vertex_template_ids {
                    match store.get_vertex_template(contained_vertex_template_id)? {
                        Some(vertex_template) => {
                            contained_vertex_templates.inner.push(vertex_template.to_variant(debug, store)?)
                        }
                        None => {}
                    }
                }
                map.into_insert("contained_vertex_templates", contained_vertex_templates);
            } else {
                let contained_vertex_template_ids: List<_> =
                    self.contained_vertex_template_ids.iter().map(|id| id.id.clone().into()).collect();
                map.into_insert("contained_vertex_template_ids", contained_vertex_template_ids);
            }
        }

        if !self.outgoing_edge_template_ids.is_empty() {
            if debug {
                let mut outgoing_edge_templates = List::new_with_capacity(self.outgoing_edge_template_ids.len());
                for outgoing_edge_template_id in &self.outgoing_edge_template_ids {
                    match store.get_edge_template(outgoing_edge_template_id)? {
                        Some(edge_template) => {
                            outgoing_edge_templates.inner.push(edge_template.to_variant(debug, store)?)
                        }
                        None => {}
                    }
                }
                map.into_insert("outgoing_edge_templates", outgoing_edge_templates);
            } else {
                let outgoing_edge_template_ids: List<_> =
                    self.outgoing_edge_template_ids.iter().map(|id| id.id.clone().into()).collect();
                map.into_insert("outgoing_edge_template_ids", outgoing_edge_template_ids);
            }
        }

        Ok(map.into())
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictVertexTemplate<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictVertexTemplate { vertex_template: self, store }
    }
}

//
// DepictVertexTemplate
//

/// Depict vertex template.
pub struct DepictVertexTemplate<'own, StoreT>
where
    StoreT: Store,
{
    vertex_template: &'own VertexTemplate,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictVertexTemplate<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "VertexTemplate")?;
        depict_id("id", Some(&self.vertex_template.template.id), false, writer, context)?;
        depict_metadata(&self.vertex_template.template.metadata, false, writer, context)?;
        depict_classes(&self.vertex_template.template.class_ids, self.store, writer, context)?;
        depict_properties(
            "property_templates",
            &self.vertex_template.template.property_templates,
            self.store,
            false,
            writer,
            context,
        )?;

        utils::depict_field(
            "contained_vertex_templates",
            false,
            writer,
            context,
            |writer, context| -> io::Result<()> {
                if self.vertex_template.contained_vertex_template_ids.is_empty() {
                    context.separate(writer)?;
                    context.theme.write_delimiter(writer, "[]")?;
                } else {
                    for (vertex_template_id, last) in
                        IterateWithLast::new(&self.vertex_template.contained_vertex_template_ids)
                    {
                        context.indent_into_thick_branch(writer, last)?;
                        match self.store.get_vertex_template(vertex_template_id).map_err(io::Error::other)? {
                            Some(vertex_template) => {
                                vertex_template
                                    .to_depict(self.store)
                                    .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                            }

                            None => {
                                vertex_template_id.depict(writer, &context.child().with_separator(false))?;
                            }
                        }
                    }
                }

                Ok(())
            },
        )?;

        utils::depict_field("outgoing_edge_templates", true, writer, context, |writer, context| -> io::Result<()> {
            if self.vertex_template.outgoing_edge_template_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (edge_template_id, last) in IterateWithLast::new(&self.vertex_template.outgoing_edge_template_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_edge_template(edge_template_id).map_err(io::Error::other)? {
                        Some(edge_template) => {
                            edge_template
                                .to_depict(self.store)
                                .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            edge_template_id.depict(writer, &context.child().with_separator(false))?;
                        }
                    }
                }
            }

            Ok(())
        })
    }
}
