use super::{super::store::*, depict::*, id::*, kind::*, metadata::*, scope::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{cli::depict::*, std::zerocopy::*},
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
    pub fn new_for(scope: Scope, id: ByteString) -> Self {
        Self::new_with(ID::new_for(Kind::Group, scope, id))
    }

    /// Constructor.
    pub fn new_with(id: ID) -> Self {
        Self {
            id,
            metadata: Default::default(),
            parent_group_ids: Default::default(),
            child_group_ids: Default::default(),
        }
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

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictGroup<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictGroup { group: self, store }
    }
}

//
// DepictGroup
//

/// Depict group.
#[allow(unused)]
pub struct DepictGroup<'own, StoreT>
where
    StoreT: Store,
{
    group: &'own Group,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictGroup<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Group")?;
        write_debug_id("id", Some(&self.group.id), false, writer, context)?;
        write_debug_metadata(&self.group.metadata, true, writer, context)?;
        Ok(())
    }
}
