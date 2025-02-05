use super::super::{super::store::*, bindings, errors::*, host::*, library::*};

use {kutil::std::immutable::*, std::path, wasmtime::component::*};

//
// DispatchPlugin
//

/// Floria dispatch plugin.
pub struct DispatchPlugin<StoreT>
where
    StoreT: 'static + Store,
{
    /// Name.
    pub name: ByteString,

    /// Host
    pub host: wasmtime::Store<PluginHost<StoreT>>,

    /// Bindings.
    pub bindings: bindings::DispatchPlugin,
}

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Clone + Send + Store,
{
    /// Constructor.
    pub fn new(
        name: ByteString,
        host: wasmtime::Store<PluginHost<StoreT>>,
        bindings: bindings::DispatchPlugin,
    ) -> Self {
        Self { name, host, bindings }
    }

    /// Constructor.
    pub fn new_from_component(
        component: Component,
        name: ByteString,
        library: &Library<StoreT>,
    ) -> Result<Self, PluginError> {
        // Host
        let mut host =
            wasmtime::Store::new(&library.environment.engine, PluginHost::new(name.clone(), library.clone()));

        // Linker
        let mut linker = Linker::new(&library.environment.engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker).map_err(PluginError::LinkWasm)?;
        bindings::DispatchPlugin::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut PluginHost<StoreT>| state)
            .map_err(PluginError::LinkWasm)?;

        // Bindings
        let bindings = bindings::DispatchPlugin::instantiate(&mut host, &component, &linker)
            .map_err(PluginError::InstantiateWasm)?;

        Ok(Self::new(name, host, bindings))
    }

    /// Constructor.
    pub fn new_from_bytes(bytes: &[u8], name: ByteString, library: &Library<StoreT>) -> Result<Self, PluginError> {
        let component = Component::from_binary(&library.environment.engine, bytes).map_err(PluginError::LoadWasm)?;
        Self::new_from_component(component, name, library)
    }

    /// Constructor.
    pub fn new_from_file<PathT>(path: PathT, name: ByteString, library: &Library<StoreT>) -> Result<Self, PluginError>
    where
        PathT: AsRef<path::Path>,
    {
        let component = Component::from_file(&library.environment.engine, path).map_err(PluginError::LoadWasm)?;
        Self::new_from_component(component, name, library)
    }
}
