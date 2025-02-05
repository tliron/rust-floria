use super::super::{super::store::*, bindings::floria::plugins::floria as host};

use {compris::annotate::*, wasmtime_wasi::*};

//
// PluginHost
//

/// Floria plugin host.
pub struct PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Name.
    pub name: String,

    /// Store.
    pub store: StoreT,

    wasi: WasiCtx,
    pub(crate) resources: ResourceTable,
}

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(name: String, store: StoreT) -> Self {
        Self {
            name,
            store,
            wasi: WasiCtxBuilder::new().inherit_stdout().inherit_stderr().build(),
            resources: ResourceTable::default(),
        }
    }
}

impl<StoreT> WasiView for PluginHost<StoreT>
where
    StoreT: Store + Send,
{
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi, table: &mut self.resources }
    }
}

impl<StoreT> host::Host for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn log(&mut self, source: String, message: String) -> Result<(), wasmtime::Error> {
        tracing::info!("[{}] {}: {}", self.name, source, message);
        Ok(())
    }

    fn get_entity(&mut self, id: host::Id) -> Result<Result<host::Any, String>, wasmtime::Error> {
        Ok(match self.store.get_entity_as_variant::<WithoutAnnotations>(&id.into())? {
            Some(value) => Ok(self.to_host_any(value)?),
            None => todo!(),
        })
    }
}
