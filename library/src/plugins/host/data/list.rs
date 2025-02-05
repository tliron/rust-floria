use super::super::{super::super::store::*, super::bindings::floria::plugins::floria as host, host::*};

use wasmtime::component::*;

//
// List
//

/// List.
pub struct List {
    /// Inner.
    pub inner: Vec<host::Any>,
}

impl List {
    /// Constructor.
    pub fn new(inner: Vec<host::Any>) -> Self {
        Self { inner }
    }
}

impl<StoreT> host::HostAnyList for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn new(&mut self, list: Vec<host::Any>) -> wasmtime::Result<Resource<List>> {
        let list = List::new(list);
        Ok(self.resources.push(list)?)
    }

    fn drop(&mut self, resource: Resource<List>) -> wasmtime::Result<()> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<List>) -> wasmtime::Result<Vec<host::Any>> {
        let list = self.resources.get(&resource)?;
        Ok(list.inner.clone())
    }

    fn length(&mut self, resource: Resource<List>) -> wasmtime::Result<u64> {
        let list = self.resources.get(&resource)?;
        Ok(list.inner.len() as u64)
    }
}
