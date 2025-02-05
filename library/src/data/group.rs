use super::{super::store::*, debug::*, id::*, kind::*, metadata::*, namespace::*};

use {
    bytestring::*,
    compris::normal::*,
    kutil_cli::debug::*,
    std::{collections::*, io},
};

//
// Group
//

/// Group.
#[derive(Clone, Debug)]
pub struct Group<AnnotatedT> {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata<AnnotatedT>,

    /// Parent group IDs.
    pub parent_group_ids: Vec<ID>,

    /// Child group IDs.
    pub child_group_ids: Vec<ID>,
}

impl<AnnotatedT> Group<AnnotatedT> {
    /// Constructor.
    pub fn new_for(namespace: Namespace, id: ByteString) -> Self
    where
        AnnotatedT: Default,
    {
        Self::new_with(ID::new_for(Kind::Group, namespace, id))
    }

    /// Constructor.
    pub fn new_with(id: ID) -> Self
    where
        AnnotatedT: Default,
    {
        Self { id, metadata: Metadata::default(), parent_group_ids: Vec::new(), child_group_ids: Vec::new() }
    }

    /// To [Variant].
    pub fn to_variant(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Clone + Default,
    {
        let mut map = BTreeMap::new();

        map.insert("kind".into(), self.id.kind.to_string().into());
        map.insert("id".into(), self.id.to_string().into());
        map.insert("metadata".into(), self.metadata.clone().into());

        map.into()
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableGroup<'own, StoreT, AnnotatedT>
    where
        StoreT: Store<AnnotatedT>,
    {
        DebuggableGroup { group: self, store }
    }
}

//
// DebuggableGroup
//

/// Debuggable group.
#[allow(unused)]
pub struct DebuggableGroup<'own, StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    group: &'own Group<AnnotatedT>,
    store: &'own StoreT,
}

impl<'own, StoreT, AnnotatedT> Debuggable for DebuggableGroup<'own, StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
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
