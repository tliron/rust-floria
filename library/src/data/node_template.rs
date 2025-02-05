use super::{super::store::*, debug::*, id::*, kind::*, namespace::*, template::*};

use {
    bytestring::*,
    compris::{annotate::*, normal::*},
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::io,
};

//
// NodeTemplate
//

/// Node template.
///
/// Equivalent to TOSCA service template, node template, or capability definition.
#[derive(Clone, Debug)]
pub struct NodeTemplate {
    /// Template.
    pub template: Template,

    /// Containing node template ID.
    pub containing_node_template_id: Option<ID>,

    /// Contained node template IDs.
    pub contained_node_template_ids: Vec<ID>,

    /// Outgoing relationship template IDs.
    pub outgoing_relationship_template_ids: Vec<ID>,
}

impl NodeTemplate {
    /// Constructor.
    pub fn new<StoreT>(namespace: Namespace, store: &StoreT) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(Kind::NodeTemplate, namespace);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, None))
    }

    /// Constructor.
    pub fn new_for(namespace: Namespace, id: ByteString, containing_node_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(Kind::NodeTemplate, namespace, id), containing_node_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, containing_node_template_id: Option<ID>) -> Self {
        Self {
            template: Template::new(id),
            containing_node_template_id,
            contained_node_template_ids: Vec::new(),
            outgoing_relationship_template_ids: Vec::new(),
        }
    }

    /// Instantiate.
    #[cfg(feature = "plugins")]
    pub fn instantiate<StoreT, ErrorRecipientT>(
        &self,
        namespace: &Namespace,
        containing_node_id: Option<ID>,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<super::Node, super::super::FloriaError>
    where
        StoreT: Store,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::FloriaError>,
    {
        let node_id = self.instantiate_nodes(namespace, containing_node_id, library, plugin_name, errors)?;
        let mut node = library
            .environment
            .store
            .get_node(&node_id)?
            .ok_or_else(|| super::super::StoreError::ID(node_id.to_string()))?;
        node.instantiate_relationships(namespace, library, plugin_name, errors)?;
        node.update(library, plugin_name, &library.environment.store, errors)?;

        Ok(node)
    }

    /// Instantiate nodes.
    #[cfg(feature = "plugins")]
    pub fn instantiate_nodes<StoreT, ErrorRecipientT>(
        &self,
        namespace: &Namespace,
        containing_node_id: Option<ID>,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<ID, super::super::FloriaError>
    where
        StoreT: Store,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::FloriaError>,
    {
        let mut node = super::node::Node {
            instance: self.template.instantiate(Kind::Node, namespace, &library.environment.store)?,
            containing_node_id,
            contained_node_ids: Vec::with_capacity(self.contained_node_template_ids.len()),
            outgoing_relationship_ids: Vec::new(),
            incoming_relationship_ids: Vec::new(),
        };

        let node_id = node.instance.id.clone();

        for contained_node_template_id in &self.contained_node_template_ids {
            match library.environment.store.get_node_template(contained_node_template_id)? {
                Some(contained_node_template) => {
                    let contained_node_id = contained_node_template.instantiate_nodes(
                        namespace,
                        Some(node_id.clone()),
                        library,
                        plugin_name,
                        errors,
                    )?;
                    node.contained_node_ids.push(contained_node_id);
                }

                None => tracing::warn!("node template not found: {}", contained_node_template_id),
            }
        }

        library.environment.store.add_node(node)?;

        Ok(node_id)
    }

    /// To [Variant].
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
            if let Some(containing_node_template_id) = &self.containing_node_template_id {
                map.into_insert("containing_node_template_id", containing_node_template_id.id.clone());
            }
        }

        if !self.contained_node_template_ids.is_empty() {
            if debug {
                let mut contained_node_templates = List::new_with_capacity(self.contained_node_template_ids.len());
                for contained_node_template_id in &self.contained_node_template_ids {
                    match store.get_node_template(contained_node_template_id)? {
                        Some(node_template) => {
                            contained_node_templates.inner.push(node_template.to_variant(debug, store)?)
                        }
                        None => {}
                    }
                }
                map.into_insert("contained_node_templates", contained_node_templates);
            } else {
                let contained_node_template_ids: List<_> =
                    self.contained_node_template_ids.iter().map(|id| id.id.clone().into()).collect();
                map.into_insert("contained_node_template_ids", contained_node_template_ids);
            }
        }

        if !self.outgoing_relationship_template_ids.is_empty() {
            if debug {
                let mut outgoing_relationship_templates =
                    List::new_with_capacity(self.outgoing_relationship_template_ids.len());
                for outgoing_relationship_template_id in &self.outgoing_relationship_template_ids {
                    match store.get_relationship_template(outgoing_relationship_template_id)? {
                        Some(relationship_template) => {
                            outgoing_relationship_templates.inner.push(relationship_template.to_variant(debug, store)?)
                        }
                        None => {}
                    }
                }
                map.into_insert("outgoing_relationship_templates", outgoing_relationship_templates);
            } else {
                let outgoing_relationship_template_ids: List<_> =
                    self.outgoing_relationship_template_ids.iter().map(|id| id.id.clone().into()).collect();
                map.into_insert("outgoing_relationship_template_ids", outgoing_relationship_template_ids);
            }
        }

        Ok(map.into())
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableNodeTemplate<'own, StoreT>
    where
        StoreT: Store,
    {
        DebuggableNodeTemplate { node_template: self, store }
    }
}

//
// DebuggableNodeTemplate
//

/// Debuggable node template.
pub struct DebuggableNodeTemplate<'own, StoreT>
where
    StoreT: Store,
{
    node_template: &'own NodeTemplate,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableNodeTemplate<'own, StoreT>
where
    StoreT: Store,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "NodeTemplate")?;
        write_debug_id("id", Some(&self.node_template.template.id), false, writer, context)?;
        write_debug_metadata(&self.node_template.template.metadata, false, writer, context)?;
        write_debug_groups(&self.node_template.template.group_ids, self.store, writer, context)?;
        write_debug_properties(
            "property_templates",
            &self.node_template.template.property_templates,
            self.store,
            false,
            writer,
            context,
        )?;

        utils::write_debug_field(
            "contained_node_templates",
            false,
            writer,
            context,
            |writer, context| -> io::Result<()> {
                if self.node_template.contained_node_template_ids.is_empty() {
                    context.separate(writer)?;
                    context.theme.write_delimiter(writer, "[]")?;
                } else {
                    for (node_template_id, last) in
                        IterateWithLast::new(&self.node_template.contained_node_template_ids)
                    {
                        context.indent_into_thick_branch(writer, last)?;
                        match self.store.get_node_template(node_template_id).map_err(io::Error::other)? {
                            Some(node_template) => {
                                node_template.to_debuggable(self.store).write_debug_for(
                                    writer,
                                    &context.child().increase_indentation_thick_branch(last),
                                )?;
                            }

                            None => {
                                node_template_id.write_debug_for(writer, &context.child().with_separator(false))?;
                            }
                        }
                    }
                }

                Ok(())
            },
        )?;

        utils::write_debug_field(
            "outgoing_relationship_templates",
            true,
            writer,
            context,
            |writer, context| -> io::Result<()> {
                if self.node_template.outgoing_relationship_template_ids.is_empty() {
                    context.separate(writer)?;
                    context.theme.write_delimiter(writer, "[]")?;
                } else {
                    for (relationship_template_id, last) in
                        IterateWithLast::new(&self.node_template.outgoing_relationship_template_ids)
                    {
                        context.indent_into_thick_branch(writer, last)?;
                        match self
                            .store
                            .get_relationship_template(relationship_template_id)
                            .map_err(io::Error::other)?
                        {
                            Some(relationship_template) => {
                                relationship_template.to_debuggable(self.store).write_debug_for(
                                    writer,
                                    &context.child().increase_indentation_thick_branch(last),
                                )?;
                            }

                            None => {
                                relationship_template_id
                                    .write_debug_for(writer, &context.child().with_separator(false))?;
                            }
                        }
                    }
                }

                Ok(())
            },
        )
    }
}
