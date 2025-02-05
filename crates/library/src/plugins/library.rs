use super::{super::store::*, environment::*, errors::*, functions::*};

use {ahash::*, std::path};

//
// Library
//

/// Plugin library.
pub struct Library<'own, StoreT>
where
    StoreT: StoreClient,
{
    /// Environment.
    pub environment: &'own Environment<StoreT>,

    /// Functions.
    pub functions: HashMap<String, FunctionsPlugin<StoreT>>,
}

impl<'own, StoreT> Library<'own, StoreT>
where
    StoreT: StoreClient,
{
    /// Constructor.
    pub fn new(environment: &'own Environment<StoreT>) -> Self {
        Self { environment, functions: HashMap::new() }
    }

    /// Add a [FunctionsPlugin].
    pub fn add_functions_plugin(&mut self, plugin_name: &str, bytes: &[u8]) -> Result<(), PluginError> {
        let functions = FunctionsPlugin::new(plugin_name.into(), self.environment, bytes)?;
        self.functions.insert(plugin_name.into(), functions);
        Ok(())
    }

    /// Load a [FunctionsPlugin].
    pub fn load_functions_plugin<PathT>(&mut self, plugin_name: &str, path: PathT) -> Result<(), PluginError>
    where
        PathT: AsRef<path::Path>,
    {
        let functions = FunctionsPlugin::new_from_file(plugin_name.into(), self.environment, path)?;
        self.functions.insert(plugin_name.into(), functions);
        Ok(())
    }

    /// Get a [FunctionsPlugin].
    pub fn get_functions_plugin(&mut self, plugin_name: &str) -> Result<&mut FunctionsPlugin<StoreT>, PluginError> {
        match self.functions.get_mut(plugin_name) {
            Some(functions) => Ok(functions),
            None => Err(PluginError::NotFound(plugin_name.into())),
        }
    }
}
