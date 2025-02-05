use super::{
    super::{super::store::*, bindings::floria::plugins::floria as host},
    host::*,
};

use {std::collections::*, wasmtime::component::*};

//
// Map
//

/// Map.
///
/// Importantly, complex map keys (which contain [Map] or [List](super::list::List)), are
/// identified by their representation ID, *not* their content. However, also note that
/// a [Clone] of the key maintains the representation IDs, so it would still be considered
/// the same key.
pub struct Map {
    // Inner.
    inner: BTreeMap<host::Any, host::Any>,
}

impl Map {
    /// Constructor.
    pub fn new(inner: BTreeMap<host::Any, host::Any>) -> Self {
        Self { inner }
    }
}

impl<StoreT> host::HostNestedMap for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn new(&mut self, key_value_pairs: Vec<(host::Any, host::Any)>) -> Result<Resource<Map>, wasmtime::Error> {
        let map = Map::new(BTreeMap::from_iter(key_value_pairs));
        Ok(self.resources.push(map)?)
    }

    fn drop(&mut self, resource: Resource<Map>) -> Result<(), wasmtime::Error> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<Map>) -> Result<Vec<(host::Any, host::Any)>, wasmtime::Error> {
        let map = self.resources.get(&resource)?;
        Ok(map.inner.clone().into_iter().collect())
    }

    fn length(&mut self, resource: Resource<Map>) -> Result<u64, wasmtime::Error> {
        let map = self.resources.get(&resource)?;
        Ok(map.inner.len() as u64)
    }
}
