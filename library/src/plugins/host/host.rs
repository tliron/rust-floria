use super::super::{super::store::*, library::*};

use {kutil::std::immutable::*, wasmtime_wasi::*};

//
// PluginHost
//

/// Floria plugin host.
pub struct PluginHost<StoreT>
where
    StoreT: 'static + Store,
{
    /// Name.
    pub name: ByteString,

    /// Library.
    pub library: Library<StoreT>,

    /// WASI context.
    pub wasi: WasiCtx,

    /// Resources.
    pub resources: ResourceTable,
}

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(name: ByteString, library: Library<StoreT>) -> Self {
        Self {
            name,
            library,
            wasi: WasiCtxBuilder::new().inherit_stdout().inherit_stderr().build(),
            resources: ResourceTable::default(),
        }
    }
}

impl<StoreT> WasiView for PluginHost<StoreT>
where
    StoreT: 'static + Send + Store,
{
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi, table: &mut self.resources }
    }
}
