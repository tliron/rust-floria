use super::super::{
    super::{data::*, store::*},
    bindings::floria::plugins::host,
};

use wasmtime_wasi::*;

//
// PluginHost
//

/// Plugins host.
pub struct PluginHost<StoreT>
where
    StoreT: StoreClient,
{
    /// Name.
    pub name: String,

    /// Store.
    pub store: StoreT,

    pub(crate) resources: ResourceTable,

    wasi: WasiCtx,
}

impl<StoreT> PluginHost<StoreT>
where
    StoreT: StoreClient,
{
    /// Constructor.
    pub fn new(name: String, store: StoreT) -> Self {
        let wasi = WasiCtxBuilder::new().inherit_stdout().build();
        Self { name, store, wasi, resources: ResourceTable::new() }
    }
}

impl<StoreT> WasiView for PluginHost<StoreT>
where
    StoreT: StoreClient,
{
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

impl<StoreT> IoView for PluginHost<StoreT>
where
    StoreT: StoreClient,
{
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resources
    }
}

impl<StoreT> host::Host for PluginHost<StoreT>
where
    StoreT: StoreClient,
{
    fn log(&mut self, source: String, message: String) -> Result<(), wasmtime::Error> {
        tracing::info!("[{}] {}: {}", self.name, source, message);
        Ok(())
    }

    fn get_node(&mut self, id: String) -> Result<Result<host::Value, String>, wasmtime::Error> {
        let id = ID::parse(Kind::Node, &id);

        Ok(match self.store.get_node(&id)? {
            Some(node) => {
                let value = node.to_value(&self.store)?;
                Ok(self.to_host_value(&value)?)
            }

            None => Err("not found".into()),
        })
    }

    fn get_relationship(&mut self, id: String) -> Result<Result<host::Value, String>, wasmtime::Error> {
        let id = ID::parse(Kind::Relationship, &id);

        Ok(match self.store.get_relationship(&id)? {
            Some(relationship) => {
                let value = relationship.to_value(&self.store)?;
                Ok(self.to_host_value(&value)?)
            }

            None => Err("not found".into()),
        })
    }
}
