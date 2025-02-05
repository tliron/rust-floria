use super::super::{super::store::*, bindings::floria::plugins::floria as host};

use wasmtime_wasi::*;

//
// PluginHost
//

/// Floria plugin host.
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
        let wasi = WasiCtxBuilder::new().inherit_stdout().inherit_stderr().build();
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

    fn get_entity(&mut self, id: host::Id) -> Result<Result<host::Value, String>, wasmtime::Error> {
        Ok(match self.store.get_entity_as_value(&id.into())? {
            Some(value) => Ok(self.to_host_value(value)?),
            None => todo!(),
        })
    }
}
