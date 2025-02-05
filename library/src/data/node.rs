use super::{super::store::*, depict::*, id::*, instance::*, kind::*, scope::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{
        cli::depict::*,
        std::{error::*, iter::*, zerocopy::*},
    },
    std::io,
};

//
// Node
//

/// Node.
#[derive(Clone, Debug)]
pub struct Node {
    /// Instance.
    pub instance: Instance,

    /// Containing node ID.
    pub containing_node_id: Option<ID>,

    /// Contained node IDs.
    pub contained_node_ids: Vec<ID>,

    /// Outgoing relationships.
    pub outgoing_relationship_ids: Vec<ID>,

    /// Incoming relationships.
    pub incoming_relationship_ids: Vec<ID>,
}

impl Node {
    /// Constructor.
    pub fn new<StoreT>(scope: Scope, origin_template_id: ID, store: &StoreT) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(Kind::Node, scope);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, Some(origin_template_id)))
    }

    /// Constructor.
    pub fn new_for(scope: Scope, id: ByteString, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(Kind::Node, scope, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self {
            instance: Instance::new_with(id, origin_template_id),
            containing_node_id: None,
            contained_node_ids: Default::default(),
            outgoing_relationship_ids: Default::default(),
            incoming_relationship_ids: Default::default(),
        }
    }

    /// To [Variant].
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
            if let Some(containing_node_id) = &self.containing_node_id {
                map.into_insert("containing_node_id", containing_node_id.to_string());
            }
        }

        if !self.contained_node_ids.is_empty() {
            if embedded {
                let mut contained_nodes = List::new_with_capacity(self.contained_node_ids.len());
                for contained_node_id in &self.contained_node_ids {
                    match store.get_node(contained_node_id)? {
                        Some(node) => contained_nodes.inner.push(node.to_variant(embedded, store)?),
                        None => {}
                    }
                }
                map.into_insert("contained_nodes", contained_nodes);
            } else {
                let contained_node_ids: List<_> =
                    self.contained_node_ids.iter().map(|id| id.to_string().into()).collect();
                map.into_insert("contained_node_ids", contained_node_ids);
            }
        }

        if !self.outgoing_relationship_ids.is_empty() {
            if embedded {
                let mut outgoing_relationships = List::new_with_capacity(self.outgoing_relationship_ids.len());
                for outgoing_relationship_id in &self.outgoing_relationship_ids {
                    if let Some(relationship) = store.get_relationship(outgoing_relationship_id)? {
                        outgoing_relationships.inner.push(relationship.to_variant(embedded, store)?);
                    }
                }
                map.into_insert("outgoing_relationships", outgoing_relationships);
            } else {
                let outgoing_relationship_ids: List<_> =
                    self.outgoing_relationship_ids.iter().map(|id| id.to_string().into()).collect();
                map.into_insert("outgoing_relationship_ids", outgoing_relationship_ids);
            }
        }

        if !embedded && !self.incoming_relationship_ids.is_empty() {
            let incoming_relationship_ids: List<_> =
                self.incoming_relationship_ids.iter().map(|id| id.to_string().into()).collect();
            map.into_insert("incoming_relationship_ids", incoming_relationship_ids);
        }

        Ok(map.into())
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictNode<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictNode { node: self, store }
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        store: &StoreT,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), super::super::FloriaError>
    where
        StoreT: Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        self.instance.update(library, plugin_name, errors)?;
        store.add_node(self.clone())?;

        for node_id in &self.contained_node_ids {
            if let Some(mut node) = store.get_node(node_id)? {
                node.update(library, plugin_name, store, errors)?;
            }
        }

        Ok(())
    }

    /// Instantiate relationships.
    #[cfg(feature = "plugins")]
    pub fn instantiate_relationships<StoreT, ErrorRecipientT>(
        &self,
        scope: &Scope,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), super::super::FloriaError>
    where
        StoreT: Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        for contained_node_id in &self.contained_node_ids {
            match library.environment.store.get_node(contained_node_id)? {
                Some(contained_node) => {
                    contained_node.instantiate_relationships(scope, library, plugin_name, errors)?;
                }

                None => tracing::warn!("node not found: {}", contained_node_id),
            }
        }

        let mut node = self.clone();

        match &node.instance.origin_template_id {
            Some(origin_template_id) => match library.environment.store.get_node_template(origin_template_id)? {
                Some(node_template) => {
                    for outgoing_relationship_template_id in &node_template.outgoing_relationship_template_ids {
                        match library.environment.store.get_relationship_template(outgoing_relationship_template_id)? {
                            Some(outgoing_relationship_template) => {
                                match outgoing_relationship_template.target_selector.select(
                                    &node.instance.id,
                                    outgoing_relationship_template_id,
                                    library,
                                    plugin_name,
                                    errors,
                                )? {
                                    Some(target_node_id) => {
                                        let outgoing_relationship_id = outgoing_relationship_template.instantiate(
                                            scope,
                                            node.instance.id.clone(),
                                            target_node_id,
                                            &library.environment.store,
                                        )?;

                                        node.outgoing_relationship_ids.push(outgoing_relationship_id);
                                    }

                                    None => errors.give(super::super::FloriaError::Instantiation(
                                        "target node not found".into(),
                                    ))?,
                                }
                            }

                            None => {
                                tracing::warn!("relationship template not found: {}", outgoing_relationship_template_id)
                            }
                        }
                    }
                }

                None => tracing::warn!("node template not found: {}", origin_template_id),
            },

            None => {}
        }

        library.environment.store.add_node(node)?;

        Ok(())
    }
}

//
// DepictNode
//

/// Depict node.
pub struct DepictNode<'own, StoreT>
where
    StoreT: Store,
{
    node: &'own Node,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictNode<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "Node")?;
        write_debug_id("id", Some(&self.node.instance.id), false, writer, context)?;
        write_debug_id("origin_template_id", self.node.instance.origin_template_id.as_ref(), false, writer, context)?;
        write_debug_metadata(&self.node.instance.metadata, false, writer, context)?;
        write_debug_groups(&self.node.instance.group_ids, self.store, writer, context)?;
        write_debug_properties("properties", &self.node.instance.properties, self.store, false, writer, context)?;

        utils::write_debug_field("contained_nodes", false, writer, context, |writer, context| -> io::Result<()> {
            if self.node.contained_node_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (node_id, last) in IterateWithLast::new(&self.node.contained_node_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_node(node_id).map_err(io::Error::other)? {
                        Some(node) => {
                            node.to_depict(self.store)
                                .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            node_id.depict(writer, &context.child().with_separator(false))?;
                        }
                    }
                }
            }

            Ok(())
        })?;

        utils::write_debug_field("outgoing_relationships", true, writer, context, |writer, context| -> io::Result<()> {
            if self.node.outgoing_relationship_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (relationship_id, last) in IterateWithLast::new(&self.node.outgoing_relationship_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_relationship(relationship_id).map_err(io::Error::other)? {
                        Some(relationship) => {
                            relationship
                                .to_depict(self.store)
                                .depict(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            relationship_id.depict(writer, &context.child().with_separator(false))?;
                        }
                    }
                }
            }

            Ok(())
        })
    }
}
