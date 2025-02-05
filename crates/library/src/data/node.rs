use super::{super::store::*, debug::*, id::*, instance::*, kind::*, namespace::*};

use {
    compris::normal::*,
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{collections::*, io},
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
    pub fn new<StoreT>(namespace: Namespace, origin_template_id: ID, store: &StoreT) -> Result<Self, StoreError>
    where
        StoreT: StoreClient,
    {
        let mut id = ID::new(Kind::Node, namespace);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, Some(origin_template_id)))
    }

    /// Constructor.
    pub fn new_for(namespace: Namespace, id: String, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(Kind::Node, namespace, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self {
            instance: Instance::new_with(id, origin_template_id),
            containing_node_id: None,
            contained_node_ids: Vec::new(),
            outgoing_relationship_ids: Vec::new(),
            incoming_relationship_ids: Vec::new(),
        }
    }

    /// To [Value].
    pub fn to_value<'own, StoreT>(&self, embedded: bool, store: &'own StoreT) -> Result<Value, StoreError>
    where
        StoreT: StoreClient,
    {
        let mut map = BTreeMap::new();

        self.instance.to_value(&mut map, embedded, store)?;

        if !embedded {
            if let Some(containing_node_id) = &self.containing_node_id {
                map.insert("containing_node_id".into(), containing_node_id.to_string().into());
            }
        }

        if !self.contained_node_ids.is_empty() {
            if embedded {
                let mut contained_nodes = Vec::with_capacity(self.contained_node_ids.len());
                for contained_node_id in &self.contained_node_ids {
                    match store.get_node(contained_node_id)? {
                        Some(node) => contained_nodes.push(node.to_value(embedded, store)?),
                        None => {}
                    }
                }
                map.insert("contained_nodes".into(), contained_nodes.into());
            } else {
                let contained_node_ids: Vec<Value> =
                    self.contained_node_ids.iter().map(|i| i.to_string().into()).collect();
                map.insert("contained_node_ids".into(), contained_node_ids.into());
            }
        }

        if !self.outgoing_relationship_ids.is_empty() {
            if embedded {
                let mut outgoing_relationships = Vec::with_capacity(self.outgoing_relationship_ids.len());
                for outgoing_relationship_id in &self.outgoing_relationship_ids {
                    match store.get_relationship(outgoing_relationship_id)? {
                        Some(relationship) => outgoing_relationships.push(relationship.to_value(embedded, store)?),
                        None => {}
                    }
                }
                map.insert("outgoing_relationships".into(), outgoing_relationships.into());
            } else {
                let outgoing_relationship_ids: Vec<Value> =
                    self.outgoing_relationship_ids.iter().map(|i| i.to_string().into()).collect();
                map.insert("outgoing_relationship_ids".into(), outgoing_relationship_ids.into());
            }
        }

        if !embedded && !self.incoming_relationship_ids.is_empty() {
            let incoming_relationship_ids: Vec<Value> =
                self.incoming_relationship_ids.iter().map(|i| i.to_string().into()).collect();
            map.insert("incoming_relationship_ids".into(), incoming_relationship_ids.into());
        }

        Ok(map.into())
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableNode<'own, StoreT>
    where
        StoreT: StoreClient,
    {
        DebuggableNode { node: self, store }
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        store: &StoreT,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), super::super::ImperativeError>
    where
        StoreT: StoreClient,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::ImperativeError>,
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
}

//
// DebuggableNode
//

/// Debuggable node.
pub struct DebuggableNode<'own, StoreT>
where
    StoreT: StoreClient,
{
    node: &'own Node,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableNode<'own, StoreT>
where
    StoreT: StoreClient,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "Node")?;
        write_debug_id("id", Some(&self.node.instance.id), false, writer, context)?;
        write_debug_id("origin_template_id", self.node.instance.origin_template_id.as_ref(), false, writer, context)?;
        write_debug_metadata(&self.node.instance.metadata, false, writer, context)?;
        write_debug_types(&self.node.instance.type_ids, self.store, writer, context)?;
        write_debug_properties("properties", &self.node.instance.properties, self.store, false, writer, context)?;

        utils::write_debug_field("contained_nodes", false, writer, context, |writer, context| -> io::Result<()> {
            if self.node.contained_node_ids.is_empty() {
                context.separate(writer)?;
                context.theme.write_delimiter(writer, "[]")?;
            } else {
                for (node_id, last) in IterateWithLast::new(&self.node.contained_node_ids) {
                    context.indent_into_thick_branch(writer, last)?;
                    match self.store.get_node(node_id).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                        Some(node) => {
                            node.to_debuggable(self.store)
                                .write_debug_for(writer, &context.child().increase_indentation_thick_branch(last))?;
                        }

                        None => {
                            write_debug_id("node_id", Some(node_id), false, writer, context)?;
                        }
                    }
                }
            }

            Ok(())
        })?;

        utils::write_debug_field(
            "outgoing_relationships",
            false,
            writer,
            context,
            |writer, context| -> io::Result<()> {
                if self.node.outgoing_relationship_ids.is_empty() {
                    context.separate(writer)?;
                    context.theme.write_delimiter(writer, "[]")?;
                } else {
                    for (relationship_id, last) in IterateWithLast::new(&self.node.outgoing_relationship_ids) {
                        context.indent_into_thick_branch(writer, last)?;
                        match self
                            .store
                            .get_relationship(relationship_id)
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                        {
                            Some(relationship) => {
                                relationship.to_debuggable(self.store).write_debug_for(
                                    writer,
                                    &context.child().increase_indentation_thick_branch(last),
                                )?;
                            }

                            None => {
                                write_debug_id("relationship_id", Some(relationship_id), true, writer, context)?;
                            }
                        }
                    }
                }

                Ok(())
            },
        )
    }
}
