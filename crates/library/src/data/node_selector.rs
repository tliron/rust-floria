use super::{
    super::{errors::*, store::*},
    call::*,
    id::*,
    namespace::*,
};

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

    /// Filter
    Filter(NodeFilter),
}

impl NodeSelector {
    /// Constructor.
    pub fn new_node(node_id: ID) -> Self {
        Self::NodeId(node_id)
    }

    /// Constructor.
    pub fn new_filter(filter: Call) -> Self {
        Self::Filter(NodeFilter::new(filter))
    }

    /// To [Value].
    pub fn to_value(&self) -> Value {
        let mut map = BTreeMap::new();

        match self {
            Self::NodeId(id) => {
                map.insert("id".into(), id.to_string().into());
            }
            Self::Filter(node_filter) => {
                map.insert("node_filter".into(), node_filter.to_value());
            }
        }

        map.into()
    }

    /// Select.
    pub fn select<StoreT>(&self, store: &StoreT) -> Result<ID, ImperativeError>
    where
        StoreT: StoreClient,
    {
        match self {
            Self::NodeId(id) => Ok(id.clone()),
            Self::Filter(node_filter) => node_filter.select(store),
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
            Self::Filter(node_filter) => node_filter.write_debug_for(writer, context),
        }
    }
}

//
// NodeFilter
//

/// Node filter.
#[derive(Clone, Debug)]
pub struct NodeFilter {
    /// Optional namespaces.
    pub namespaces: Option<Vec<Namespace>>,

    /// Filter.
    pub filter: Call,
}

impl NodeFilter {
    /// Constructor.
    pub fn new(filter: Call) -> Self {
        Self { namespaces: None, filter }
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

        map.insert("filter".into(), self.filter.to_value());

        map.into()
    }

    /// Select.
    pub fn select<StoreT>(&self, store: &StoreT) -> Result<ID, ImperativeError>
    where
        StoreT: StoreClient,
    {
        let nodes = store.get_nodes(self.namespaces.clone())?;
        for node in &nodes {
            return Ok(node.instance.id.clone());
        }
        Err(StoreError::ID("not found".into()).into())
    }
}

impl Debuggable for NodeFilter {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match &self.namespaces {
            Some(_namespaces) => todo!(),
            None => self.filter.write_debug_for(writer, context),
        }
    }
}
