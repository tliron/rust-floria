use super::{
    super::{
        super::{super::store::*, bindings::floria::plugins::floria as host, errors::*},
        host::*,
    },
    list::*,
    map::*,
};

use {compris::normal, std::collections::*};

impl<StoreT> PluginHost<StoreT>
where
    StoreT: Store,
{
    /// Convert a [normal::Variant] to a [host::Any].
    pub fn to_any<AnnotatedT>(&mut self, value: normal::Variant<AnnotatedT>) -> Result<host::Any, PluginError> {
        match value {
            normal::Variant::Undefined => Ok(host::Any::Null),
            normal::Variant::Null(_) => Ok(host::Any::Null),
            normal::Variant::Integer(integer) => Ok(host::Any::Integer(integer.inner)),
            normal::Variant::UnsignedInteger(unsigned_integer) => {
                Ok(host::Any::UnsignedInteger(unsigned_integer.inner))
            }
            normal::Variant::Float(float) => Ok(host::Any::Float(float.into())),
            normal::Variant::Boolean(boolean) => Ok(host::Any::Boolean(boolean.inner)),
            normal::Variant::Text(text) => Ok(host::Any::Text(text.into())),
            normal::Variant::Blob(blob) => Ok(host::Any::Blob(blob.into())),

            normal::Variant::List(list) => {
                let mut host_list = Vec::with_capacity(list.inner.len());
                for value in list {
                    host_list.push(self.to_any(value)?);
                }

                let host_list = self.resources.push(List::new(host_list))?;
                Ok(host::Any::AnyList(host_list))
            }

            normal::Variant::Map(map) => {
                let mut host_map = BTreeMap::default();
                for (key, value) in map {
                    host_map.insert(self.to_any(key)?, self.to_any(value)?);
                }

                let host_map = self.resources.push(Map::new(host_map))?;
                Ok(host::Any::AnyMap(host_map))
            }
        }
    }
}
