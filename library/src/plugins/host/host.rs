use super::super::{super::store::*, bindings::floria::plugins::floria as host};

use {
    std::{fmt, marker::*},
    wasmtime_wasi::{p2::*, *},
};

//
// PluginHost
//

/// Floria plugin host.
pub struct PluginHost<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    /// Name.
    pub name: String,

    /// Store.
    pub store: StoreT,

    wasi: WasiCtx,
    pub(crate) resources: ResourceTable,
    annotations: PhantomData<AnnotatedT>,
}

impl<StoreT, AnnotatedT> PluginHost<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    /// Constructor.
    pub fn new(name: String, store: StoreT) -> Self {
        Self {
            name,
            store,
            wasi: WasiCtxBuilder::new().inherit_stdout().inherit_stderr().build(),
            resources: ResourceTable::new(),
            annotations: PhantomData,
        }
    }
}

impl<StoreT, AnnotatedT> WasiView for PluginHost<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
    AnnotatedT: Send,
{
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

impl<StoreT, AnnotatedT> IoView for PluginHost<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
    AnnotatedT: Send,
{
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resources
    }
}

impl<StoreT, AnnotatedT> host::Host for PluginHost<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
    AnnotatedT: Clone + fmt::Debug + Default,
{
    fn log(&mut self, source: String, message: String) -> Result<(), wasmtime::Error> {
        tracing::info!("[{}] {}: {}", self.name, source, message);
        Ok(())
    }

    fn get_entity(&mut self, id: host::Id) -> Result<Result<host::Any, String>, wasmtime::Error> {
        Ok(match self.store.get_entity_as_value(&id.into())? {
            Some(value) => Ok(self.to_host_any(value)?),
            None => todo!(),
        })
    }
}
