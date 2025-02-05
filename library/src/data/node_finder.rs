use super::{call::*, id::*, prefix::*};

use {
    compris::{annotate::*, normal::*},
    kutil::cli::depict::*,
    std::io,
};

//
// NodeFinder
//

/// Node finder.
#[derive(Clone, Debug)]
pub struct NodeFinder {
    /// Optional prefixes.
    pub prefixes: Option<Vec<Prefix>>,

    /// Finder.
    pub finder: Call,
}

impl NodeFinder {
    /// Constructor.
    pub fn new(filter: Call) -> Self {
        Self { prefixes: None, finder: filter }
    }

    /// To [Variant].
    pub fn to_variant<AnnotatedT>(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = Map::default();

        if let Some(prefixes) = &self.prefixes {
            let mut prefixes_variant = List::new_with_capacity(prefixes.len());
            for prefix in prefixes {
                let prefix: List<_> = prefix.iter().map(|segment| segment.clone().into()).collect();
                prefixes_variant.inner.push(prefix.into());
            }
            map.into_insert("prefixes", prefixes_variant);
        }

        map.into_insert("filter", self.finder.to_variant());

        map.into()
    }

    /// Find.
    #[cfg(feature = "plugins")]
    pub fn find<StoreT, ErrorRecipientT>(
        &self,
        source_node_id: &ID,
        _relationship_template_id: &ID,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<ID>, super::super::FloriaError>
    where
        StoreT: Clone + Send + super::super::Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        use super::{super::plugins::*, kind::*};
        use kutil::unwrap_or_give;

        let site = Site::new(source_node_id.clone(), None);
        Ok(unwrap_or_give!(
            self.finder.evaluate::<_, WithoutAnnotations>(&site, library, plugin_name).map(Some),
            errors,
            None
        )
        .map(|id| super::id::ID::parse(Kind::Node, &id.to_string())))
    }
}

impl Depict for NodeFinder {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match &self.prefixes {
            Some(_prefixes) => todo!(),
            None => self.finder.depict(writer, context),
        }
    }
}
