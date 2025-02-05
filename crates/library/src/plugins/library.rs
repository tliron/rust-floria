use super::{super::store::*, dispatch::*, environment::*, errors::*};

use {
    kutil_std::collections::*,
    std::{fmt, path},
};

//
// Library
//

/// Plugin library.
pub struct Library<'own, StoreT, AnnotatedT>
where
    StoreT: 'static + Store<AnnotatedT>,
    AnnotatedT: 'static,
{
    /// Environment.
    pub environment: &'own Environment<StoreT, AnnotatedT>,

    /// Dispatch.
    pub dispatch: FastHashMap<String, DispatchPlugin<StoreT, AnnotatedT>>,
}

impl<'own, StoreT, AnnotatedT> Library<'own, StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    /// Constructor.
    pub fn new(environment: &'own Environment<StoreT, AnnotatedT>) -> Self {
        Self { environment, dispatch: FastHashMap::new() }
    }

    /// Add a [DispatchPlugin].
    pub fn add_dispatch_plugin(&mut self, plugin_name: &str, bytes: &[u8]) -> Result<(), PluginError>
    where
        AnnotatedT: Clone + fmt::Debug + Default + Send,
    {
        let dispatch = DispatchPlugin::new(plugin_name.into(), self.environment, bytes)?;
        self.dispatch.insert(plugin_name.into(), dispatch);
        Ok(())
    }

    /// Load a [DispatchPlugin].
    pub fn load_dispatch_plugin<PathT>(&mut self, plugin_name: &str, path: PathT) -> Result<(), PluginError>
    where
        AnnotatedT: Clone + fmt::Debug + Default + Send,
        PathT: AsRef<path::Path>,
    {
        let dispatch = DispatchPlugin::new_from_file(plugin_name.into(), self.environment, path)?;
        self.dispatch.insert(plugin_name.into(), dispatch);
        Ok(())
    }

    /// Get a [DispatchPlugin].
    pub fn get_dispatch_plugin(
        &mut self,
        plugin_name: &str,
    ) -> Result<&mut DispatchPlugin<StoreT, AnnotatedT>, PluginError> {
        self.dispatch.get_mut(plugin_name).ok_or_else(|| PluginError::NotFound(plugin_name.into()))
    }
}
