use super::{call::*, id::*, namespace::*};

use {
    compris::{annotate::*, normal::*},
    kutil_cli::debug::*,
    std::io,
};

//
// NodeFinder
//

/// Node finder.
#[derive(Clone, Debug)]
pub struct NodeFinder {
    /// Optional namespaces.
    pub namespaces: Option<Vec<Namespace>>,

    /// Finder.
    pub finder: Call,
}

impl NodeFinder {
    /// Constructor.
    pub fn new(filter: Call) -> Self {
        Self { namespaces: None, finder: filter }
    }

    /// To [Variant].
    pub fn to_variant<AnnotatedT>(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = Map::default();

        if let Some(namespaces) = &self.namespaces {
            let mut namespaces_variant = List::new_with_capacity(namespaces.len());
            for namespace in namespaces {
                let namespace: List<_> = namespace.iter().map(|segment| segment.clone().into()).collect();
                namespaces_variant.inner.push(namespace.into());
            }
            map.into_insert("namespaces", namespaces_variant);
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
        StoreT: super::super::Store,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::FloriaError>,
    {
        use kutil_std::error::*;
        let site = super::super::plugins::Site::new(source_node_id.clone(), None);
        Ok(self
            .finder
            .evaluate::<_, WithoutAnnotations>(&site, library, plugin_name)
            .map(Some)
            .give_or(None, errors)?
            .map(|id| super::id::ID::parse(super::kind::Kind::Node, &id.to_string())))
    }
}

impl Debuggable for NodeFinder {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match &self.namespaces {
            Some(_namespaces) => todo!(),
            None => self.finder.write_debug_for(writer, context),
        }
    }
}
