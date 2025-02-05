use super::super::{
    super::store::*,
    bindings::{self, exports::floria::plugins::dispatch::Site},
    environment::*,
    errors::*,
    host::*,
};

use {compris::normal::*, std::path, wasmtime::component::*};

//
// DispatchPlugin
//

/// Floria dispatch plugin.
pub struct DispatchPlugin<StoreT>
where
    StoreT: 'static + Store,
{
    /// Name.
    pub name: String,

    pub(crate) store: wasmtime::Store<PluginHost<StoreT>>,
    pub(crate) bindings: bindings::DispatchPlugin,
}

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Clone + Send + Store,
{
    /// Constructor.
    pub fn new(name: String, environment: &Environment<StoreT>, bytes: &[u8]) -> Result<Self, PluginError> {
        let component = Component::from_binary(&environment.engine, bytes).map_err(PluginError::LoadWasm)?;
        Self::new_with(name, environment, component)
    }

    /// Constructor.
    pub fn new_with(
        name: String,
        environment: &Environment<StoreT>,
        component: Component,
    ) -> Result<Self, PluginError> {
        // Linker
        let mut linker = Linker::new(&environment.engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker).map_err(PluginError::LinkWasm)?;
        bindings::DispatchPlugin::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut PluginHost<StoreT>| state)
            .map_err(PluginError::LinkWasm)?;

        // Store
        let mut store =
            wasmtime::Store::new(&environment.engine, PluginHost::new(name.clone(), environment.store.clone()));

        // Bindings
        let bindings = bindings::DispatchPlugin::instantiate(&mut store, &component, &linker)
            .map_err(PluginError::InstantiateWasm)?;

        Ok(Self { name, store, bindings })
    }

    /// Constructor.
    pub fn new_from_file<PathT>(
        name: String,
        environment: &Environment<StoreT>,
        path: PathT,
    ) -> Result<Self, PluginError>
    where
        PathT: AsRef<path::Path>,
    {
        let component = Component::from_file(&environment.engine, path).map_err(PluginError::LoadWasm)?;
        Self::new_with(name, environment, component)
    }

    /// Dispatch.
    pub fn dispatch<AnnotatedT>(
        &mut self,
        name: &str,
        arguments: Vec<Variant<AnnotatedT>>,
        site: &Site,
    ) -> Result<Variant<AnnotatedT>, PluginError>
    where
        AnnotatedT: Default,
    {
        let mut dispatch_arguments = Vec::with_capacity(arguments.len());
        for argument in arguments.into_iter() {
            dispatch_arguments.push(self.to_guest_any(argument)?);
        }

        let value = self
            .bindings
            .floria_plugins_dispatch()
            .call_dispatch(&mut self.store, name, &dispatch_arguments, site)
            .map_err(PluginError::CallWasm)?
            .map_err(PluginError::Dispatch)?;

        Ok(self.from_guest_value(value)?)
    }
}
