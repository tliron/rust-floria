use super::{call::*, id::*, namespace::*};

use {
    compris::normal::*,
    kutil_cli::debug::*,
    std::{collections::*, io},
};

//
// NodeFinder
//

/// Node finder.
#[derive(Clone, Debug)]
pub struct NodeFinder<AnnotatedT> {
    /// Optional namespaces.
    pub namespaces: Option<Vec<Namespace>>,

    /// Finder.
    pub finder: Call<AnnotatedT>,
}

impl<AnnotatedT> NodeFinder<AnnotatedT> {
    /// Constructor.
    pub fn new(filter: Call<AnnotatedT>) -> Self {
        Self { namespaces: None, finder: filter }
    }

    /// To [Variant].
    pub fn to_variant(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Clone + Default,
    {
        let mut map = BTreeMap::new();

        if let Some(namespaces) = &self.namespaces {
            let mut namespaces_value = Vec::<Variant<AnnotatedT>>::with_capacity(namespaces.len());
            for namespace in namespaces {
                let namespace: Vec<Variant<AnnotatedT>> =
                    namespace.iter().map(|segment| segment.clone().into()).collect();
                namespaces_value.push(namespace.into());
            }
            map.insert("namespaces".into(), namespaces_value.into());
        }

        map.insert("filter".into(), self.finder.to_variant());

        map.into()
    }

    /// Find.
    #[cfg(feature = "plugins")]
    pub fn find<StoreT, ErrorRecipientT>(
        &self,
        source_node_id: &ID,
        _relationship_template_id: &ID,
        library: &mut super::super::plugins::Library<StoreT, AnnotatedT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<ID>, super::super::FloriaError>
    where
        AnnotatedT: Clone + Default,
        StoreT: super::super::Store<AnnotatedT>,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::FloriaError>,
    {
        use kutil_std::error::*;
        let site = super::super::plugins::Site::new(source_node_id.clone(), None);
        Ok(self
            .finder
            .evaluate(&site, library, plugin_name)
            .map(Some)
            .give_or(None, errors)?
            .map(|id| super::id::ID::parse(super::kind::Kind::Node, &id.to_string())))
    }
}

impl<AnnotatedT> Debuggable for NodeFinder<AnnotatedT> {
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
