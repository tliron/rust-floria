use super::super::{
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

impl<StoreT> host::HostAnyMap for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn new(&mut self, key_value_pairs: Vec<(host::Any, host::Any)>) -> wasmtime::Result<Resource<Map>> {
        let map = Map::new(BTreeMap::from_iter(key_value_pairs));
        Ok(self.resources.push(map)?)
    }

    fn drop(&mut self, resource: Resource<Map>) -> wasmtime::Result<()> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<Map>) -> wasmtime::Result<Vec<(host::Any, host::Any)>> {
        let map = self.resources.get(&resource)?;
        Ok(map.inner.clone().into_iter().collect())
    }

    fn length(&mut self, resource: Resource<Map>) -> wasmtime::Result<u64> {
        let map = self.resources.get(&resource)?;
        Ok(map.inner.len() as u64)
    }
}
