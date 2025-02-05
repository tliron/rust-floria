use super::{call::*, id::*, namespace::*};

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
pub enum NodeSelector {
    /// Node ID.
    NodeId(ID),

    /// Finder
    Finder(NodeFinder),
}

impl NodeSelector {
    /// Constructor.
    pub fn new_node(node_id: ID) -> Self {
        Self::NodeId(node_id)
    }

    /// Constructor.
    pub fn new_finder(finder: Call) -> Self {
        Self::Finder(NodeFinder::new(finder))
    }

    /// To [Value].
    pub fn to_value(&self) -> Value {
        let mut map = BTreeMap::new();

        match self {
            Self::NodeId(id) => {
                map.insert("id".into(), id.to_string().into());
            }
            Self::Finder(node_filter) => {
                map.insert("node_filter".into(), node_filter.to_value());
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
    ) -> Result<Option<ID>, super::super::ImperativeError>
    where
        StoreT: super::super::StoreClient,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::ImperativeError>,
    {
        match self {
            Self::NodeId(id) => Ok(Some(id.clone())),
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
            Self::NodeId(id) => id.write_debug_for(writer, context),
            Self::Finder(node_filter) => node_filter.write_debug_for(writer, context),
        }
    }
}

//
// NodeFinder
//

/// Node filter.
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

    /// To [Value].
    pub fn to_value(&self) -> Value {
        let mut map = BTreeMap::new();

        if let Some(namespaces) = &self.namespaces {
            let mut namespaces_ = Vec::<Value>::with_capacity(namespaces.len());
            for namespace in namespaces {
                let namespace: Vec<Value> = namespace.iter().map(|n| n.clone().into()).collect();
                namespaces_.push(namespace.into());
            }
            map.insert("namespaces".into(), namespaces_.into());
        }

        map.insert("filter".into(), self.finder.to_value());

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
    ) -> Result<Option<ID>, super::super::ImperativeError>
    where
        StoreT: super::super::StoreClient,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::ImperativeError>,
    {
        use kutil_std::error::*;
        let site = super::super::plugins::Site::new(source_node_id.clone(), "".into());
        match self.finder.evaluate(&site, library, plugin_name).map(|v| Some(v)).give_or(None, errors)? {
            Some(node_id) => Ok(Some(super::id::ID::parse(super::kind::Kind::Node, &node_id.to_string()))),
            None => Ok(None),
        }
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
