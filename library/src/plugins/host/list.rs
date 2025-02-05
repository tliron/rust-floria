use super::{super::super::store::*, super::bindings::floria::plugins::floria as host, host::*};

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

impl<StoreT> host::HostNestedList for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn new(&mut self, list: Vec<host::Any>) -> Result<Resource<List>, wasmtime::Error> {
        let list = List::new(list);
        Ok(self.resources.push(list)?)
    }

    fn drop(&mut self, resource: Resource<List>) -> Result<(), wasmtime::Error> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<List>) -> Result<Vec<host::Any>, wasmtime::Error> {
        let list = self.resources.get(&resource)?;
        Ok(list.inner.clone())
    }

    fn length(&mut self, resource: Resource<List>) -> Result<u64, wasmtime::Error> {
        let list = self.resources.get(&resource)?;
        Ok(list.inner.len() as u64)
    }
}
