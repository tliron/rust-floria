use super::{
    super::{super::store::*, bindings::floria::plugins::floria as host, errors::*},
    host::*,
    list::*,
    map::*,
};

use {compris::normal, std::collections::*};

impl<StoreT, AnnotatedT> PluginHost<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    /// Convert a [normal::Value] to a [host::Value].
    pub fn to_host_value(&mut self, value: normal::Value<AnnotatedT>) -> Result<host::Value, PluginError> {
        match value {
            normal::Value::Nothing => Ok(host::Value::Null),

            normal::Value::Null(_) => Ok(host::Value::Null),

            normal::Value::Integer(integer) => Ok(host::Value::Integer(integer.inner)),

            normal::Value::UnsignedInteger(unsigned_integer) => {
                Ok(host::Value::UnsignedInteger(unsigned_integer.inner))
            }

            normal::Value::Float(float) => Ok(host::Value::Float(float.into())),

            normal::Value::Boolean(boolean) => Ok(host::Value::Boolean(boolean.inner)),

            normal::Value::Text(text) => Ok(host::Value::Text(text.into())),

            normal::Value::Blob(blob) => Ok(host::Value::Blob(blob.into())),

            normal::Value::List(list) => {
                let mut host_list = Vec::with_capacity(list.inner.len());
                for value in list {
                    host_list.push(self.to_host_value(value)?);
                }

                let host_list = self.resources.push(List::new(host_list)).map_err(PluginError::WasmResource)?;
                Ok(host::Value::NestedList(host_list))
            }

            normal::Value::Map(map) => {
                let mut host_map = BTreeMap::new();
                for (key, value) in map {
                    host_map.insert(self.to_host_value(key)?, self.to_host_value(value)?);
                }

                let host_map = self.resources.push(Map::new(host_map)).map_err(PluginError::WasmResource)?;
                Ok(host::Value::NestedMap(host_map))
            }
        }
    }
}
