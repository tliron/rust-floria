use super::{call::*, id::*, node_finder::*};

use {
    compris::normal::*,
    kutil_cli::debug::*,
    std::{collections::*, io},
};

//
// NodeSelector
//

/// Node selector.
#[derive(Clone, Debug)]
pub enum NodeSelector<AnnotatedT> {
    /// Node ID.
    NodeID(ID),

    /// Finder
    Finder(NodeFinder<AnnotatedT>),
}

impl<AnnotatedT> NodeSelector<AnnotatedT> {
    /// Constructor.
    pub fn new_node(node_id: ID) -> Self {
        Self::NodeID(node_id)
    }

    /// Constructor.
    pub fn new_finder(finder: Call<AnnotatedT>) -> Self {
        Self::Finder(NodeFinder::new(finder))
    }

    /// To [Variant].
    pub fn to_variant(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Clone + Default,
    {
        let mut map = BTreeMap::new();

        match self {
            Self::NodeID(id) => {
                map.insert("id".into(), id.to_string().into());
            }
            Self::Finder(node_filter) => {
                map.insert("node_filter".into(), node_filter.to_variant());
            }
        }

        map.into()
    }

    /// Select.
    #[cfg(feature = "plugins")]
    pub fn select<StoreT, ErrorRecipientT>(
        &self,
        source_node_id: &ID,
        relationship_template_id: &ID,
        library: &mut super::super::plugins::Library<StoreT, AnnotatedT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<ID>, super::super::FloriaError>
    where
        AnnotatedT: Clone + Default,
        StoreT: super::super::Store<AnnotatedT>,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::FloriaError>,
    {
        match self {
            Self::NodeID(id) => Ok(Some(id.clone())),
            Self::Finder(node_finder) => {
                node_finder.find(source_node_id, relationship_template_id, library, plugin_name, errors)
            }
        }
    }
}

impl<AnnotatedT> Debuggable for NodeSelector<AnnotatedT> {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::NodeID(id) => id.write_debug_for(writer, context),
            Self::Finder(node_filter) => node_filter.write_debug_for(writer, context),
        }
    }
}
