use super::super::{super::super::store::*, super::bindings::floria::plugins::floria as host, host::*};

use wasmtime::component::*;

//
// Call
//

/// Call.
pub struct Call {
    /// Name.
    pub name: String,

    /// Arguments.
    pub arguments: Vec<host::Any>,
}

impl Call {
    /// Constructor.
    pub fn new(name: String, arguments: Vec<host::Any>) -> Self {
        Self { name, arguments }
    }
}

impl<StoreT> host::HostAnyCall for PluginHost<StoreT>
where
    StoreT: Store,
{
    fn new(&mut self, name: String, arguments: Vec<host::Any>) -> wasmtime::Result<Resource<Call>> {
        let call = Call::new(name, arguments);
        Ok(self.resources.push(call)?)
    }

    fn drop(&mut self, resource: Resource<Call>) -> wasmtime::Result<()> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<Call>) -> wasmtime::Result<(String, Vec<host::Any>)> {
        let call = self.resources.get(&resource)?;
        Ok((call.name.clone(), call.arguments.clone()))
    }
}
