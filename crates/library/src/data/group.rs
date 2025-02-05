use super::{super::store::*, debug::*, id::*, kind::*, metadata::*, namespace::*};

use {
    compris::normal::*,
    kutil_cli::debug::*,
    std::{collections::*, io},
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
    pub fn new_for(namespace: Namespace, id: String) -> Self {
        Self::new_with(ID::new_for(Kind::Group, namespace, id))
    }

    /// Constructor.
    pub fn new_with(id: ID) -> Self {
        Self { id, metadata: Metadata::new(), parent_group_ids: Vec::new(), child_group_ids: Vec::new() }
    }

    /// To [Value].
    pub fn to_value(&self) -> Value {
        let mut map = BTreeMap::new();

        map.insert("kind".into(), self.id.kind.to_string().into());
        map.insert("id".into(), self.id.to_string().into());
        map.insert("metadata".into(), self.metadata.clone().into());

        map.into()
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableGroup<'own, StoreT>
    where
        StoreT: StoreClient,
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
    StoreT: StoreClient,
{
    group: &'own Group,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableGroup<'own, StoreT>
where
    StoreT: StoreClient,
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
