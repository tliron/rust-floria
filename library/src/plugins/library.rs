use super::{super::store::*, dispatch::*, environment::*, errors::*};

use {kutil::std::collections::*, std::path};

//
// Library
//

/// Plugin library.
pub struct Library<'environment, StoreT>
where
    StoreT: 'static + Store,
{
    /// Environment.
    pub environment: &'environment Environment<StoreT>,

    /// Dispatch.
    pub dispatch: FastHashMap<String, DispatchPlugin<StoreT>>,
}

impl<'environment, StoreT> Library<'environment, StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(environment: &'environment Environment<StoreT>) -> Self {
        Self { environment, dispatch: Default::default() }
    }

    /// Add a [DispatchPlugin].
    pub fn add_dispatch_plugin(&mut self, plugin_name: String, bytes: &[u8]) -> Result<(), PluginError>
    where
        StoreT: Clone + Send,
    {
        let dispatch = DispatchPlugin::new(plugin_name.clone(), self.environment, bytes)?;
        self.dispatch.insert(plugin_name, dispatch);
        Ok(())
    }

    /// Load a [DispatchPlugin].
    pub fn load_dispatch_plugin<PathT>(&mut self, plugin_name: String, path: PathT) -> Result<(), PluginError>
    where
        StoreT: Clone + Send,
        PathT: AsRef<path::Path>,
    {
        let dispatch = DispatchPlugin::new_from_file(plugin_name.clone(), self.environment, path)?;
        self.dispatch.insert(plugin_name, dispatch);
        Ok(())
    }

    /// Get a [DispatchPlugin].
    pub fn get_dispatch_plugin(&mut self, plugin_name: &str) -> Result<&mut DispatchPlugin<StoreT>, PluginError> {
        self.dispatch.get_mut(plugin_name).ok_or_else(|| PluginError::NotFound(plugin_name.into()))
    }
}
