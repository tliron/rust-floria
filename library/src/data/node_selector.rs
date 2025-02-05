use super::{call::*, id::*, node_finder::*};

use {
    compris::{annotate::*, normal::*},
    kutil_cli::debug::*,
    std::io,
};

//
// NodeSelector
//

/// Node selector.
#[derive(Clone, Debug)]
pub enum NodeSelector {
    /// Node ID.
    NodeID(ID),

    /// Finder
    Finder(NodeFinder),
}

impl NodeSelector {
    /// Constructor.
    pub fn new_node(node_id: ID) -> Self {
        Self::NodeID(node_id)
    }

    /// Constructor.
    pub fn new_finder(finder: Call) -> Self {
        Self::Finder(NodeFinder::new(finder))
    }

    /// To [Variant].
    pub fn to_variant<AnnotatedT>(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = Map::default();

        match self {
            Self::NodeID(id) => {
                map.into_insert("id", id.to_string());
            }
            Self::Finder(node_filter) => {
                map.into_insert("node_filter", node_filter.to_variant());
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
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<ID>, super::super::FloriaError>
    where
        StoreT: super::super::Store,
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

impl Debuggable for NodeSelector {
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
