use super::super::{super::store::*, bindings::exports::floria::plugins::dispatcher, errors::*, functions::*};

use compris::normal;

impl<StoreT> FunctionsPlugin<StoreT>
where
    StoreT: StoreClient,
{
    /// Convert a [normal::Value] to a [dispatcher::Value].
    pub fn to_guest_value(&mut self, value: normal::Value) -> Result<dispatcher::Value, PluginError> {
        match value {
            normal::Value::Nothing | normal::Value::Null(_) => Ok(dispatcher::Value::Null),

            normal::Value::Integer(integer) => Ok(dispatcher::Value::Integer(integer.value)),

            normal::Value::UnsignedInteger(unsigned_integer) => {
                Ok(dispatcher::Value::UnsignedInteger(unsigned_integer.value))
            }

            normal::Value::Float(float) => Ok(dispatcher::Value::Float(float.value.into())),

            normal::Value::Boolean(boolean) => Ok(dispatcher::Value::Boolean(boolean.value)),

            normal::Value::Text(text) => Ok(dispatcher::Value::Text(text.value)),

            normal::Value::Bytes(bytes) => Ok(dispatcher::Value::Bytes(bytes.value)),

            normal::Value::List(list) => {
                let mut vector = Vec::with_capacity(list.value.len());
                for value in list.into_iter() {
                    vector.push(self.to_guest_value(value)?);
                }

                let nested_list_access = self.functions.floria_plugins_dispatcher().nested_list();
                let resource =
                    nested_list_access.call_constructor(&mut self.store, &vector).map_err(PluginError::CallWasm)?;
                Ok(dispatcher::Value::NestedList(resource))
            }

            normal::Value::Map(map) => {
                let mut key_value_pairs = Vec::with_capacity(map.value.len());
                for (key, value) in map.into_iter() {
                    key_value_pairs.push((self.to_guest_value(key)?, self.to_guest_value(value)?));
                }

                let nested_map_access = self.functions.floria_plugins_dispatcher().nested_map();
                let resource = nested_map_access
                    .call_constructor(&mut self.store, &key_value_pairs)
                    .map_err(PluginError::CallWasm)?;
                Ok(dispatcher::Value::NestedMap(resource))
            }
        }
    }

    /// Convert a [dispatcher::Value] to a [normal::Value].
    pub fn from_guest_value(&mut self, value: dispatcher::Value) -> Result<normal::Value, PluginError> {
        match value {
            dispatcher::Value::Null => Ok(normal::Null::new().into()),

            dispatcher::Value::Integer(integer) => Ok(normal::Integer::new(integer).into()),

            dispatcher::Value::UnsignedInteger(unsigned_integer) => {
                Ok(normal::UnsignedInteger::new(unsigned_integer).into())
            }

            dispatcher::Value::Float(float) => Ok(normal::Float::new(float).into()),

            dispatcher::Value::Boolean(boolean) => Ok(normal::Boolean::new(boolean).into()),

            dispatcher::Value::Text(text) => Ok(normal::Text::new(text).into()),

            dispatcher::Value::Bytes(bytes) => Ok(normal::Bytes::new(bytes).into()),

            dispatcher::Value::NestedList(resource) => {
                let nested_list_access = self.functions.floria_plugins_dispatcher().nested_list();
                let vector = nested_list_access.call_get(&mut self.store, resource).map_err(PluginError::CallWasm)?;
                resource.resource_drop(&mut self.store).map_err(PluginError::CallWasm)?;

                let mut normal_list = Vec::with_capacity(vector.len());
                for value in vector {
                    normal_list.push(self.from_guest_value(value)?);
                }

                Ok(normal_list.into())
            }

            dispatcher::Value::NestedMap(resource) => {
                let nested_map_access = self.functions.floria_plugins_dispatcher().nested_map();
                let key_value_pairs =
                    nested_map_access.call_get(&mut self.store, resource).map_err(PluginError::CallWasm)?;
                resource.resource_drop(&mut self.store).map_err(PluginError::CallWasm)?;

                let mut normal_map = normal::Map::new();
                for (key, value) in key_value_pairs {
                    normal_map.value.insert(self.from_guest_value(key)?, self.from_guest_value(value)?);
                }

                Ok(normal_map.into())
            }
        }
    }
}
