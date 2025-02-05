use super::{super::super::store::*, super::bindings::floria::plugins::floria as host, host::*};

use wasmtime::component::*;

//
// List
//

/// List.
pub struct List {
    /// Inner.
    pub inner: Vec<host::Value>,
}

impl List {
    /// Constructor.
    pub fn new(inner: Vec<host::Value>) -> Self {
        Self { inner }
    }
}

impl<StoreT, AnnotatedT> host::HostNestedList for PluginHost<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    fn new(&mut self, list: Vec<host::Value>) -> Result<Resource<List>, wasmtime::Error> {
        let list = List::new(list);
        Ok(self.resources.push(list)?)
    }

    fn drop(&mut self, resource: Resource<List>) -> Result<(), wasmtime::Error> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<List>) -> Result<Vec<host::Value>, wasmtime::Error> {
        let list = self.resources.get(&resource)?;
        Ok(list.inner.clone())
    }

    fn length(&mut self, resource: Resource<List>) -> Result<u64, wasmtime::Error> {
        let list = self.resources.get(&resource)?;
        Ok(list.inner.len() as u64)
    }
}
