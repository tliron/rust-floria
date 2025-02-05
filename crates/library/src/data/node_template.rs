use super::{
    super::{errors::*, store::*},
    debug::*,
    id::*,
    kind::*,
    namespace::*,
    node::*,
    template::*,
};

use {
    compris::normal::*,
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{collections::*, io},
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
        StoreT: StoreClient,
    {
        let mut id = ID::new(Kind::NodeTemplate, namespace);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, None))
    }

    /// Constructor.
    pub fn new_for(namespace: Namespace, id: String, containing_node_template_id: Option<ID>) -> Self {
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
    pub fn instantiate<StoreT>(
        &self,
        namespace: &Namespace,
        containing_node_id: Option<ID>,
        store: &StoreT,
    ) -> Result<ID, ImperativeError>
    where
        StoreT: StoreClient,
    {
        let mut node = Node {
            instance: self.template.instantiate(Kind::Node, namespace, store)?,
            containing_node_id,
            contained_node_ids: Vec::with_capacity(self.contained_node_template_ids.len()),
            outgoing_relationship_ids: Vec::new(),
            incoming_relationship_ids: Vec::new(),
        };

        let node_id = node.instance.id.clone();

        for contained_node_template_id in &self.contained_node_template_ids {
            match store.get_node_template(contained_node_template_id)? {
                Some(contained_node_template) => {
                    let contained_node_id =
                        contained_node_template.instantiate(namespace, Some(node_id.clone()), store)?;
                    node.contained_node_ids.push(contained_node_id);
                }

                None => tracing::warn!("node template not found: {}", contained_node_template_id),
            }
        }

        for outgoing_relationship_template_id in &self.outgoing_relationship_template_ids {
            match store.get_relationship_template(outgoing_relationship_template_id)? {
                Some(outgoing_relationship_template) => {
                    let target_node_id = outgoing_relationship_template.target_selector.select(store)?;
                    let outgoing_relationship_id = outgoing_relationship_template.instantiate(
                        namespace,
                        node_id.clone(),
                        target_node_id,
                        store,
                    )?;
                    node.outgoing_relationship_ids.push(outgoing_relationship_id);
                }

                None => tracing::warn!("relationship template not found: {}", outgoing_relationship_template_id),
            }
        }

        store.add_node(node)?;

        Ok(node_id)
    }

    /// To [Value].
    pub fn to_value<'own, StoreT>(&self, debug: bool, store: &'own StoreT) -> Result<Value, StoreError>
    where
        StoreT: StoreClient,
    {
        let mut map = BTreeMap::new();

        self.template.to_value(&mut map, debug, store)?;

        if !debug {
            if let Some(containing_node_template_id) = &self.containing_node_template_id {
                map.insert("containing_node_template_id".into(), containing_node_template_id.id.clone().into());
            }
        }

        if !self.contained_node_template_ids.is_empty() {
            if debug {
                let mut contained_node_templates = Vec::with_capacity(self.contained_node_template_ids.len());
                for contained_node_template_id in &self.contained_node_template_ids {
                    match store.get_node_template(contained_node_template_id)? {
                        Some(node_template) => contained_node_templates.push(node_template.to_value(debug, store)?),
                        None => {}
                    }
                }
                map.insert("contained_node_templates".into(), contained_node_templates.into());
            } else {
                let contained_node_template_ids: Vec<Value> =
                    self.contained_node_template_ids.iter().map(|i| i.id.clone().into()).collect();
                map.insert("contained_node_template_ids".into(), contained_node_template_ids.into());
            }
        }

        if !self.outgoing_relationship_template_ids.is_empty() {
            if debug {
                let mut outgoing_relationship_templates =
                    Vec::with_capacity(self.outgoing_relationship_template_ids.len());
                for outgoing_relationship_template_id in &self.outgoing_relationship_template_ids {
                    match store.get_relationship_template(outgoing_relationship_template_id)? {
                        Some(relationship_template) => {
                            outgoing_relationship_templates.push(relationship_template.to_value(debug, store)?)
                        }
                        None => {}
                    }
                }
                map.insert("outgoing_relationship_templates".into(), outgoing_relationship_templates.into());
            } else {
                let outgoing_relationship_template_ids: Vec<Value> =
                    self.outgoing_relationship_template_ids.iter().map(|i| i.id.clone().into()).collect();
                map.insert("outgoing_relationship_template_ids".into(), outgoing_relationship_template_ids.into());
            }
        }

        Ok(map.into())
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableNodeTemplate<'own, StoreT>
    where
        StoreT: StoreClient,
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
    StoreT: StoreClient,
{
    node_template: &'own NodeTemplate,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableNodeTemplate<'own, StoreT>
where
    StoreT: StoreClient,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.theme.write_heading(writer, "NodeTemplate")?;
        write_debug_id("id", Some(&self.node_template.template.id), false, writer, context)?;
        write_debug_metadata(&self.node_template.template.metadata, false, writer, context)?;
        write_debug_types(&self.node_template.template.type_ids, self.store, writer, context)?;
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
                        match self
                            .store
                            .get_node_template(node_template_id)
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                        {
                            Some(node_template) => {
                                node_template.to_debuggable(self.store).write_debug_for(
                                    writer,
                                    &context.child().increase_indentation_thick_branch(last),
                                )?;
                            }

                            None => {
                                write_debug_id("node_template_id", Some(node_template_id), false, writer, context)?;
                            }
                        }
                    }
                }

                Ok(())
            },
        )?;

        utils::write_debug_field(
            "outgoing_relationship_templates",
            false,
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
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                        {
                            Some(relationship_template) => {
                                relationship_template.to_debuggable(self.store).write_debug_for(
                                    writer,
                                    &context.child().increase_indentation_thick_branch(last),
                                )?;
                            }

                            None => {
                                write_debug_id(
                                    "relationship_template_id",
                                    Some(relationship_template_id),
                                    true,
                                    writer,
                                    context,
                                )?;
                            }
                        }
                    }
                }

                Ok(())
            },
        )
    }
}
