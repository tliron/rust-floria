use super::{call::*, directory::*};

use {
    compris::{annotate::*, normal::*},
    kutil::cli::depict::*,
    std::io,
};

//
// VertexFinder
//

/// Vertex finder.
#[derive(Clone, Debug)]
pub struct VertexFinder {
    /// Optional directories.
    pub directories: Option<Vec<Directory>>,

    /// Finder.
    pub finder: Call,
}

impl VertexFinder {
    /// Constructor.
    pub fn new(filter: Call) -> Self {
        Self { directories: None, finder: filter }
    }

    /// To Compris variant.
    pub fn to_variant<AnnotatedT>(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = Map::default();

        if let Some(directories) = &self.directories {
            let mut directories_variant = List::new_with_capacity(directories.len());
            for directory in directories {
                let directory: List<_> = directory.into_iter().map(|segment| segment.clone().into()).collect();
                directories_variant.inner.push(directory.into());
            }
            map.into_insert("directories", directories_variant);
        }

        map.into_insert("finder", &self.finder);

        map.into()
    }

    /// Find.
    #[cfg(feature = "plugins")]
    pub fn find<StoreT, ErrorRecipientT>(
        &self,
        source_vertex_id: &super::ID,
        _edge_template_id: &super::ID,
        library: &mut super::super::plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<super::ID>, super::super::FloriaError>
    where
        StoreT: Clone + Send + super::super::Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        use super::{super::plugins::*, kind::*};
        use kutil::unwrap_or_give;

        let site = Site::new(source_vertex_id.clone(), Default::default());
        Ok(unwrap_or_give!(self.finder.evaluate::<_, WithoutAnnotations>(&site, library).map(Some), errors, None)
            .map(|id| super::id::ID::parse(Kind::Vertex, &id.to_string())))
    }
}

impl Depict for VertexFinder {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match &self.directories {
            Some(_directories) => todo!(),
            None => self.finder.depict(writer, context),
        }
    }
}
