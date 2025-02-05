use super::{super::store::*, debug::*, id::*, instance::*};

use {
    compris::normal::*,
    kutil_cli::debug::*,
    std::{collections::*, io},
};

//
// Relationship
//

/// Relationship.
#[derive(Clone, Debug)]
pub struct Relationship<AnnotatedT> {
    /// Instance.
    pub instance: Instance<AnnotatedT>,

    /// Source node ID.
    pub source_node_id: ID,

    /// Target node ID.
    pub target_node_id: ID,
}

impl<AnnotatedT> Relationship<AnnotatedT> {
    /// To [Value].
    pub fn to_value<'own, StoreT>(&self, embedded: bool, store: &'own StoreT) -> Result<Value<AnnotatedT>, StoreError>
    where
        AnnotatedT: Clone + Default,
        StoreT: Store<AnnotatedT>,
    {
        let mut map = BTreeMap::new();

        self.instance.to_value(&mut map, embedded, store)?;

        if !embedded {
            map.insert("source_node_id".into(), self.source_node_id.to_string().into());
        }

        map.insert("target_node_id".into(), self.target_node_id.to_string().into());

        Ok(map.into())
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(
        &'own self,
        store: &'own StoreT,
    ) -> DebuggableRelationship<'own, StoreT, AnnotatedT>
    where
        StoreT: Store<AnnotatedT>,
    {
        DebuggableRelationship { relationship: self, store }
    }
}

//
// DebuggableRelationship
//

/// Debuggable relationship.
pub struct DebuggableRelationship<'own, StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    relationship: &'own Relationship<AnnotatedT>,
    store: &'own StoreT,
}

impl<'own, StoreT, AnnotatedT> Debuggable for DebuggableRelationship<'own, StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Relationship")?;
        write_debug_id("id", Some(&self.relationship.instance.id), false, writer, context)?;
        write_debug_id(
            "origin_template_id",
            self.relationship.instance.origin_template_id.as_ref(),
            false,
            writer,
            context,
        )?;
        write_debug_metadata(&self.relationship.instance.metadata, false, writer, context)?;
        write_debug_groups(&self.relationship.instance.group_ids, self.store, writer, context)?;
        write_debug_properties(
            "properties",
            &self.relationship.instance.properties,
            self.store,
            false,
            writer,
            context,
        )?;
        write_debug_id("target_node_id", Some(&self.relationship.target_node_id), true, writer, context)
    }
}
