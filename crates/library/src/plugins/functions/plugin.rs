use super::{
    super::{super::store::*, bindings, environment::*, errors::*, host::*},
    site::*,
};

use {compris::normal::*, std::path, wasmtime::component::*};

//
// FunctionsPlugin
//

/// Clout functions plugin.
pub struct FunctionsPlugin<StoreT>
where
    StoreT: StoreClient,
{
    /// Name.
    pub name: String,

    pub(crate) store: wasmtime::Store<PluginHost<StoreT>>,
    pub(crate) functions: bindings::Functions,
}

impl<StoreT> FunctionsPlugin<StoreT>
where
    StoreT: StoreClient,
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
        wasmtime_wasi::add_to_linker_sync(&mut linker).map_err(PluginError::LinkWasm)?;
        bindings::Functions::add_to_linker(&mut linker, |state: &mut PluginHost<_>| state)
            .map_err(PluginError::LinkWasm)?;

        // Store
        let mut store =
            wasmtime::Store::new(&environment.engine, PluginHost::new(name.clone(), environment.store.clone()));

        // Bindings
        let functions =
            bindings::Functions::instantiate(&mut store, &component, &linker).map_err(PluginError::InstantiateWasm)?;

        Ok(Self { name, store, functions })
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
    pub fn dispatch(&mut self, name: &str, arguments: Vec<Value>, site: &Site) -> Result<Value, PluginError> {
        let mut guest_arguments = Vec::with_capacity(arguments.len());
        for argument in arguments.into_iter() {
            guest_arguments.push(self.to_guest_value(argument)?);
        }

        let value = self
            .functions
            .floria_plugins_dispatcher()
            .call_dispatch(&mut self.store, name, &guest_arguments, site)
            .map_err(PluginError::CallWasm)?
            .map_err(PluginError::Dispatch)?;

        Ok(self.from_guest_value(value)?)
    }
}
