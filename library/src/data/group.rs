use super::{super::store::*, debug::*, id::*, kind::*, metadata::*, namespace::*};

use {
    bytestring::*,
    compris::{annotate::*, normal::*},
    kutil_cli::debug::*,
    std::io,
};

//
// Group
//

/// Group.
#[derive(Clone, Debug)]
pub struct Group {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata,

    /// Parent group IDs.
    pub parent_group_ids: Vec<ID>,

    /// Child group IDs.
    pub child_group_ids: Vec<ID>,
}

impl Group {
    /// Constructor.
    pub fn new_for(namespace: Namespace, id: ByteString) -> Self {
        Self::new_with(ID::new_for(Kind::Group, namespace, id))
    }

    /// Constructor.
    pub fn new_with(id: ID) -> Self {
        Self { id, metadata: Metadata::default(), parent_group_ids: Vec::new(), child_group_ids: Vec::new() }
    }

    /// To [Variant].
    pub fn to_variant<AnnotatedT>(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = Map::default();

        map.into_insert("kind", self.id.kind.to_string());
        map.into_insert("id", self.id.to_string());
        map.into_insert("metadata", self.metadata.clone().into_annotated());

        map.into()
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableGroup<'own, StoreT>
    where
        StoreT: Store,
    {
        DebuggableGroup { group: self, store }
    }
}

//
// DebuggableGroup
//

/// Debuggable group.
#[allow(unused)]
pub struct DebuggableGroup<'own, StoreT>
where
    StoreT: Store,
{
    group: &'own Group,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableGroup<'own, StoreT>
where
    StoreT: Store,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Group")?;
        write_debug_id("id", Some(&self.group.id), false, writer, context)?;
        write_debug_metadata(&self.group.metadata, true, writer, context)?;
        Ok(())
    }
}
