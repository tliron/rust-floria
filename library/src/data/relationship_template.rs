use super::{super::store::*, debug::*, id::*, kind::*, namespace::*, node_selector::*, relationship::*, template::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{cli::debug::*, std::zerocopy::*},
    std::io,
};

//
// RelationshipTemplate
//

/// Relationship template.
///
/// Equivalent to TOSCA requirement.
#[derive(Clone, Debug)]
pub struct RelationshipTemplate {
    /// Template.
    pub template: Template,

    /// Containing source node template ID.
    pub containing_source_node_template_id: ID,

    /// Target selector.
    /// TODO: required? node/capability filter?
    pub target_selector: NodeSelector,
}

impl RelationshipTemplate {
    /// Constructor.
    pub fn new<StoreT>(
        namespace: Namespace,
        containing_source_node_template_id: ID,
        target_selector: NodeSelector,
        store: &StoreT,
    ) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(Kind::RelationshipTemplate, namespace);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, containing_source_node_template_id, target_selector))
    }

    /// Constructor.
    pub fn new_for(
        namespace: Namespace,
        id: ByteString,
        containing_source_node_template_id: ID,
        target_selector: NodeSelector,
    ) -> Self {
        Self::new_with(
            ID::new_for(Kind::RelationshipTemplate, namespace, id),
            containing_source_node_template_id,
            target_selector,
        )
    }

    /// Constructor.
    pub fn new_with(id: ID, containing_source_node_template_id: ID, target_selector: NodeSelector) -> Self {
        Self { template: Template::new(id), containing_source_node_template_id, target_selector }
    }

    /// Instantiate.
    pub fn instantiate<StoreT>(
        &self,
        namespace: &Namespace,
        source_node_id: ID,
        target_node_id: ID,
        store: &StoreT,
    ) -> Result<ID, StoreError>
    where
        StoreT: Store,
    {
        let relationship = Relationship {
            instance: self.template.instantiate(Kind::Relationship, namespace, store)?,
            source_node_id,
            target_node_id,
        };

        let relationship_id = relationship.instance.id.clone();
        store.add_relationship(relationship)?;

        Ok(relationship_id)
    }

    /// To [Variant].
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

        map.into_insert("containing_source_node_template_id", self.containing_source_node_template_id.id.clone());
        map.into_insert("target_selector", self.target_selector.to_variant());

        Ok(map.into())
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableRelationshipTemplate<'own, StoreT>
    where
        StoreT: Store,
    {
        DebuggableRelationshipTemplate { relationship_template: self, store }
    }
}

//
// DebuggableRelationshipTemplate
//

/// Debuggable relationship template.
pub struct DebuggableRelationshipTemplate<'own, StoreT>
where
    StoreT: Store,
{
    relationship_template: &'own RelationshipTemplate,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableRelationshipTemplate<'own, StoreT>
where
    StoreT: Store,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "RelationshipTemplate")?;
        write_debug_id("id", Some(&self.relationship_template.template.id), false, writer, context)?;
        write_debug_metadata(&self.relationship_template.template.metadata, false, writer, context)?;
        write_debug_groups(&self.relationship_template.template.group_ids, self.store, writer, context)?;
        write_debug_properties(
            "property_templates",
            &self.relationship_template.template.property_templates,
            self.store,
            false,
            writer,
            context,
        )?;

        utils::write_debug_field("target_selector", true, writer, context, |writer, context| -> io::Result<()> {
            self.relationship_template.target_selector.write_debug_for(writer, context)
        })
    }
}
