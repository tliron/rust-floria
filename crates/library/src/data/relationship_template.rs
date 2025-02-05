use super::{super::store::*, debug::*, id::*, kind::*, namespace::*, node_selector::*, relationship::*, template::*};

use {
    compris::normal::*,
    kutil_cli::debug::*,
    std::{collections::*, io},
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
        StoreT: StoreClient,
    {
        let mut id = ID::new(Kind::RelationshipTemplate, namespace);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, containing_source_node_template_id, target_selector))
    }

    /// Constructor.
    pub fn new_for(
        namespace: Namespace,
        id: String,
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
        StoreT: StoreClient,
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

    /// To [Value].
    pub fn to_value<'own, StoreT>(&self, debug: bool, store: &'own StoreT) -> Result<Value, StoreError>
    where
        StoreT: StoreClient,
    {
        let mut map = BTreeMap::new();

        self.template.to_value(&mut map, debug, store)?;

        map.insert(
            "containing_source_node_template_id".into(),
            self.containing_source_node_template_id.id.clone().into(),
        );

        map.insert("target_selector".into(), self.target_selector.to_value());

        Ok(map.into())
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableRelationshipTemplate<'own, StoreT>
    where
        StoreT: StoreClient,
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
    StoreT: StoreClient,
{
    relationship_template: &'own RelationshipTemplate,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableRelationshipTemplate<'own, StoreT>
where
    StoreT: StoreClient,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "RelationshipTemplate")?;
        write_debug_id("id", Some(&self.relationship_template.template.id), false, writer, context)?;
        write_debug_metadata(&self.relationship_template.template.metadata, false, writer, context)?;
        write_debug_types(&self.relationship_template.template.type_ids, self.store, writer, context)?;
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
