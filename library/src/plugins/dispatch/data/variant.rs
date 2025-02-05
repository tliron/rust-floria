use super::super::{
    super::{super::store::*, bindings::exports::floria::plugins::dispatch, errors::*},
    plugin::*,
};

use compris::normal;

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Store,
{
    /// Convert a [normal::Variant] to a [dispatch::Any].
    pub fn to_any<AnnotatedT>(&mut self, value: normal::Variant<AnnotatedT>) -> Result<dispatch::Any, PluginError> {
        match value {
            normal::Variant::Undefined | normal::Variant::Null(_) => Ok(dispatch::Any::Null),
            normal::Variant::Integer(integer) => Ok(dispatch::Any::Integer(integer.inner)),
            normal::Variant::UnsignedInteger(unsigned_integer) => {
                Ok(dispatch::Any::UnsignedInteger(unsigned_integer.inner))
            }
            normal::Variant::Float(float) => Ok(dispatch::Any::Float(float.into())),
            normal::Variant::Boolean(boolean) => Ok(dispatch::Any::Boolean(boolean.inner)),
            normal::Variant::Text(text) => Ok(dispatch::Any::Text(text.into())),
            normal::Variant::Blob(blob) => Ok(dispatch::Any::Blob(blob.into())),

            normal::Variant::List(list) => {
                let mut vector = Vec::with_capacity(list.inner.len());
                for value in list.into_iter() {
                    vector.push(self.to_any(value)?);
                }

                let any_list = self.bindings.floria_plugins_dispatch().any_list();
                let resource = any_list.call_constructor(&mut self.host, &vector).map_err(PluginError::CallWasm)?;
                Ok(dispatch::Any::AnyList(resource))
            }

            normal::Variant::Map(map) => {
                let mut key_value_pairs = Vec::with_capacity(map.inner.len());
                for (key, value) in map.into_iter() {
                    key_value_pairs.push((self.to_any(key)?, self.to_any(value)?));
                }

                let any_map = self.bindings.floria_plugins_dispatch().any_map();
                let resource =
                    any_map.call_constructor(&mut self.host, &key_value_pairs).map_err(PluginError::CallWasm)?;
                Ok(dispatch::Any::AnyMap(resource))
            }
        }
    }

    /// Convert a [dispatch::Any] to a [normal::Variant].
    pub fn from_any<AnnotatedT>(&mut self, value: dispatch::Any) -> Result<normal::Variant<AnnotatedT>, PluginError>
    where
        AnnotatedT: Default,
    {
        match value {
            dispatch::Any::Null => Ok(normal::Null::default().into()),
            dispatch::Any::Integer(integer) => Ok(normal::Integer::from(integer).into()),
            dispatch::Any::UnsignedInteger(unsigned_integer) => {
                Ok(normal::UnsignedInteger::from(unsigned_integer).into())
            }
            dispatch::Any::Float(float) => Ok(normal::Float::from(float).into()),
            dispatch::Any::Boolean(boolean) => Ok(normal::Boolean::from(boolean).into()),
            dispatch::Any::Text(text) => Ok(normal::Text::from(text).into()),
            dispatch::Any::Blob(blob) => Ok(normal::Blob::from(blob).into()),

            dispatch::Any::AnyList(resource) => {
                let any_list = self.bindings.floria_plugins_dispatch().any_list();
                let vector = any_list.call_get(&mut self.host, resource).map_err(PluginError::CallWasm)?;
                resource.resource_drop(&mut self.host).map_err(PluginError::CallWasm)?;

                let mut list = Vec::with_capacity(vector.len());
                for value in vector {
                    list.push(self.from_any(value)?);
                }

                Ok(list.into())
            }

            dispatch::Any::AnyMap(resource) => {
                let any_map = self.bindings.floria_plugins_dispatch().any_map();
                let key_value_pairs = any_map.call_get(&mut self.host, resource).map_err(PluginError::CallWasm)?;
                resource.resource_drop(&mut self.host).map_err(PluginError::CallWasm)?;

                let mut map = normal::Map::default();
                for (key, value) in key_value_pairs {
                    map.inner.insert(self.from_any(key)?, self.from_any(value)?);
                }

                Ok(map.into())
            }

            dispatch::Any::AnyCall(resource) => {
                let any_call = self.bindings.floria_plugins_dispatch().any_call();
                let (name, arguments) = any_call.call_get(&mut self.host, resource).map_err(PluginError::CallWasm)?;
                resource.resource_drop(&mut self.host).map_err(PluginError::CallWasm)?;

                let mut list = normal::List::new_with_capacity(arguments.len());
                for argument in arguments {
                    list.inner.push(self.from_any(argument)?);
                }

                let mut map = normal::Map::default();
                map.into_insert(String::from("$") + &name, list);

                Ok(map.into())
            }
        }
    }
}
