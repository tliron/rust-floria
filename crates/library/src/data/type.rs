use super::{super::store::*, debug::*, id::*, kind::*, metadata::*, namespace::*};

use {
    compris::normal::*,
    kutil_cli::debug::*,
    std::{collections::*, io},
};

//
// Type
//

/// Type.
#[derive(Clone, Debug)]
pub struct Type {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata,

    /// Parent type IDs.
    pub parent_type_ids: Vec<ID>,

    /// Child type IDs.
    pub child_type_ids: Vec<ID>,
}

impl Type {
    /// Constructor.
    pub fn new_for(namespace: Namespace, id: String) -> Self {
        Self::new_with(ID::new_for(Kind::Type, namespace, id))
    }

    /// Constructor.
    pub fn new_with(id: ID) -> Self {
        Self { id, metadata: Metadata::new(), parent_type_ids: Vec::new(), child_type_ids: Vec::new() }
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
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableType<'own, StoreT>
    where
        StoreT: StoreClient,
    {
        DebuggableType { type_: self, store }
    }
}

//
// DebuggableType
//

/// Debuggable type.
#[allow(unused)]
pub struct DebuggableType<'own, StoreT>
where
    StoreT: StoreClient,
{
    type_: &'own Type,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableType<'own, StoreT>
where
    StoreT: StoreClient,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Type")?;
        write_debug_id("id", Some(&self.type_.id), false, writer, context)?;
        write_debug_metadata(&self.type_.metadata, true, writer, context)?;
        Ok(())
    }
}
