use super::{super::store::*, depict::*, directory::*, edge::*, id::*, kind::*, template::*, vertex_selector::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{cli::depict::*, std::immutable::*},
    std::io,
};

//
// EdgeTemplate
//

/// Edge template.
#[derive(Clone, Debug)]
pub struct EdgeTemplate {
    /// Template.
    pub template: Template,

    /// Containing source vertex template ID.
    pub containing_source_vertex_template_id: ID,

    /// Target selector.
    /// TODO: required? vertex/capability filter?
    pub target_selector: VertexSelector,
}

impl EdgeTemplate {
    /// Constructor.
    pub fn new<StoreT>(
        directory: Directory,
        containing_source_vertex_template_id: ID,
        target_selector: VertexSelector,
        store: &StoreT,
    ) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(Kind::EdgeTemplate, directory);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, containing_source_vertex_template_id, target_selector))
    }

    /// Constructor.
    pub fn new_for(
        directory: Directory,
        id: ByteString,
        containing_source_vertex_template_id: ID,
        target_selector: VertexSelector,
    ) -> Self {
        Self::new_with(
            ID::new_for(Kind::EdgeTemplate, directory, id),
            containing_source_vertex_template_id,
            target_selector,
        )
    }

    /// Constructor.
    pub fn new_with(id: ID, containing_source_vertex_template_id: ID, target_selector: VertexSelector) -> Self {
        Self { template: Template::new(id), containing_source_vertex_template_id, target_selector }
    }

    /// Instantiate.
    pub fn instantiate<StoreT>(
        &self,
        directory: &Directory,
        source_vertex_id: ID,
        target_vertex_id: ID,
        store: &StoreT,
    ) -> Result<ID, StoreError>
    where
        StoreT: Store,
    {
        let edge = Edge {
            instance: self.template.instantiate(Kind::Edge, directory, store)?,
            source_vertex_id,
            target_vertex_id,
        };

        let edge_id = edge.instance.id.clone();
        store.add_edge(edge)?;

        Ok(edge_id)
    }

    /// To Compris variant.
    pub fn to_variant<'own, StoreT, AnnotatedT>(
        &self,
        debug: bool,
        store: &'own StoreT,
    ) -> Result<Variant<AnnotatedT>, StoreError>
    where
        StoreT: Store,
        AnnotatedT: Annotated + Default + Clone,
    {
        let mut map = Map::default();

        self.template.to_variant(&mut map, debug, store)?;

        map.into_insert("containing_source_vertex_template_id", self.containing_source_vertex_template_id.id.clone());
        map.into_insert("target_selector", self.target_selector.to_variant());

        Ok(map.into())
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictEdgeTemplate<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictEdgeTemplate { edge_template: self, store }
    }
}

//
// DepictEdgeTemplate
//

/// Depict edge template.
pub struct DepictEdgeTemplate<'own, StoreT>
where
    StoreT: Store,
{
    edge_template: &'own EdgeTemplate,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictEdgeTemplate<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "EdgeTemplate")?;
        depict_id("id", Some(&self.edge_template.template.id), false, writer, context)?;
        depict_metadata(&self.edge_template.template.metadata, false, writer, context)?;
        depict_classes(&self.edge_template.template.class_ids, self.store, writer, context)?;
        depict_properties(
            "property_templates",
            &self.edge_template.template.property_templates,
            self.store,
            false,
            writer,
            context,
        )?;

        utils::depict_field("target_selector", true, writer, context, |writer, context| -> io::Result<()> {
            self.edge_template.target_selector.depict(writer, context)
        })
    }
}
