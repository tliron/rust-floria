use super::{super::store::*, dispatch::*, environment::*, errors::*};

use {kutil::std::collections::*, std::path};

//
// Library
//

/// Plugin library.
pub struct Library<'own, StoreT>
where
    StoreT: 'static + Store,
{
    /// Environment.
    pub environment: &'own Environment<StoreT>,

    /// Dispatch.
    pub dispatch: FastHashMap<String, DispatchPlugin<StoreT>>,
}

impl<'own, StoreT> Library<'own, StoreT>
where
    StoreT: Store,
{
    /// Constructor.
    pub fn new(environment: &'own Environment<StoreT>) -> Self {
        Self { environment, dispatch: FastHashMap::default() }
    }

    /// Add a [DispatchPlugin].
    pub fn add_dispatch_plugin(&mut self, plugin_name: &str, bytes: &[u8]) -> Result<(), PluginError> {
        let dispatch = DispatchPlugin::new(plugin_name.into(), self.environment, bytes)?;
        self.dispatch.insert(plugin_name.into(), dispatch);
        Ok(())
    }

    /// Load a [DispatchPlugin].
    pub fn load_dispatch_plugin<PathT>(&mut self, plugin_name: &str, path: PathT) -> Result<(), PluginError>
    where
        PathT: AsRef<path::Path>,
    {
        let dispatch = DispatchPlugin::new_from_file(plugin_name.into(), self.environment, path)?;
        self.dispatch.insert(plugin_name.into(), dispatch);
        Ok(())
    }

    /// Get a [DispatchPlugin].
    pub fn get_dispatch_plugin(&mut self, plugin_name: &str) -> Result<&mut DispatchPlugin<StoreT>, PluginError> {
        self.dispatch.get_mut(plugin_name).ok_or_else(|| PluginError::NotFound(plugin_name.into()))
    }
}
