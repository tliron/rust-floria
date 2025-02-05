use super::{
    super::{super::store::*, bindings::exports::floria::plugins::dispatch, errors::*},
    plugin::*,
};

use compris::normal;

impl<StoreT, AnnotatedT> DispatchPlugin<StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    /// Convert a [normal::Value] to a [dispatch::Value].
    pub fn to_guest_value(&mut self, value: normal::Value<AnnotatedT>) -> Result<dispatch::Value, PluginError> {
        match value {
            normal::Value::Nothing | normal::Value::Null(_) => Ok(dispatch::Value::Null),

            normal::Value::Integer(integer) => Ok(dispatch::Value::Integer(integer.inner)),

            normal::Value::UnsignedInteger(unsigned_integer) => {
                Ok(dispatch::Value::UnsignedInteger(unsigned_integer.inner))
            }

            normal::Value::Float(float) => Ok(dispatch::Value::Float(float.into())),

            normal::Value::Boolean(boolean) => Ok(dispatch::Value::Boolean(boolean.inner)),

            normal::Value::Text(text) => Ok(dispatch::Value::Text(text.into())),

            normal::Value::Blob(blob) => Ok(dispatch::Value::Blob(blob.into())),

            normal::Value::List(list) => {
                let mut vector = Vec::with_capacity(list.inner.len());
                for value in list.into_iter() {
                    vector.push(self.to_guest_value(value)?);
                }

                let nested_list_access = self.bindings.floria_plugins_dispatch().nested_list();
                let resource =
                    nested_list_access.call_constructor(&mut self.store, &vector).map_err(PluginError::CallWasm)?;
                Ok(dispatch::Value::NestedList(resource))
            }

            normal::Value::Map(map) => {
                let mut key_value_pairs = Vec::with_capacity(map.inner.len());
                for (key, value) in map.into_iter() {
                    key_value_pairs.push((self.to_guest_value(key)?, self.to_guest_value(value)?));
                }

                let nested_map_access = self.bindings.floria_plugins_dispatch().nested_map();
                let resource = nested_map_access
                    .call_constructor(&mut self.store, &key_value_pairs)
                    .map_err(PluginError::CallWasm)?;
                Ok(dispatch::Value::NestedMap(resource))
            }
        }
    }

    /// Convert a [dispatch::Value] to a [normal::Value].
    pub fn from_guest_value(&mut self, value: dispatch::Value) -> Result<normal::Value<AnnotatedT>, PluginError>
    where
        AnnotatedT: Default,
    {
        match value {
            dispatch::Value::Null => Ok(normal::Null::default().into()),

            dispatch::Value::Integer(integer) => Ok(normal::Integer::new(integer).into()),

            dispatch::Value::UnsignedInteger(unsigned_integer) => {
                Ok(normal::UnsignedInteger::new(unsigned_integer).into())
            }

            dispatch::Value::Float(float) => Ok(normal::Float::from(float).into()),

            dispatch::Value::Boolean(boolean) => Ok(normal::Boolean::new(boolean).into()),

            dispatch::Value::Text(text) => Ok(normal::Text::from(text).into()),

            dispatch::Value::Blob(blob) => Ok(normal::Blob::from(blob).into()),

            dispatch::Value::NestedList(resource) => {
                let nested_list_access = self.bindings.floria_plugins_dispatch().nested_list();
                let vector = nested_list_access.call_get(&mut self.store, resource).map_err(PluginError::CallWasm)?;
                resource.resource_drop(&mut self.store).map_err(PluginError::CallWasm)?;

                let mut normal_list = Vec::with_capacity(vector.len());
                for value in vector {
                    normal_list.push(self.from_guest_value(value)?);
                }

                Ok(normal_list.into())
            }

            dispatch::Value::NestedMap(resource) => {
                let nested_map_access = self.bindings.floria_plugins_dispatch().nested_map();
                let key_value_pairs =
                    nested_map_access.call_get(&mut self.store, resource).map_err(PluginError::CallWasm)?;
                resource.resource_drop(&mut self.store).map_err(PluginError::CallWasm)?;

                let mut normal_map = normal::Map::default();
                for (key, value) in key_value_pairs {
                    normal_map.inner.insert(self.from_guest_value(key)?, self.from_guest_value(value)?);
                }

                Ok(normal_map.into())
            }
        }
    }
}
