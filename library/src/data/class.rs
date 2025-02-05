use super::{super::store::*, depict::*, directory::*, id::*, kind::*, metadata::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{cli::depict::*, std::immutable::*},
    std::io,
};

//
// Class
//

/// Class.
#[derive(Clone, Debug)]
pub struct Class {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata,

    /// TODO: Parent class IDs.
    pub parent_class_ids: Vec<ID>,

    /// TODO: Child class IDs.
    pub child_class_ids: Vec<ID>,
}

impl Class {
    /// Constructor.
    pub fn new_for(directory: Directory, id: ByteString) -> Self {
        Self::new_with(ID::new_for(Kind::Class, directory, id))
    }

    /// Constructor.
    pub fn new_with(id: ID) -> Self {
        Self {
            id,
            metadata: Default::default(),
            parent_class_ids: Default::default(),
            child_class_ids: Default::default(),
        }
    }

    /// To Compris variant.
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
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictClass<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictClass { class: self, store }
    }
}

//
// DepictClass
//

/// Depict class.
#[allow(unused)]
pub struct DepictClass<'own, StoreT>
where
    StoreT: Store,
{
    class: &'own Class,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictClass<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.theme.write_heading(writer, "Class")?;
        depict_id("id", Some(&self.class.id), false, writer, context)?;
        depict_metadata(&self.class.metadata, true, writer, context)?;
        Ok(())
    }
}
