use super::{
    super::{super::store::*, bindings::exports::floria::plugins::dispatch, errors::*},
    plugin::*,
};

use compris::normal;

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Store,
{
    /// Convert a [normal::Variant] to a [dispatch::Any].
    pub fn to_guest_any<AnnotatedT>(
        &mut self,
        value: normal::Variant<AnnotatedT>,
    ) -> Result<dispatch::Any, PluginError> {
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
                    vector.push(self.to_guest_any(value)?);
                }

                let nested_list_access = self.bindings.floria_plugins_dispatch().nested_list();
                let resource =
                    nested_list_access.call_constructor(&mut self.store, &vector).map_err(PluginError::CallWasm)?;
                Ok(dispatch::Any::NestedList(resource))
            }

            normal::Variant::Map(map) => {
                let mut key_value_pairs = Vec::with_capacity(map.inner.len());
                for (key, value) in map.into_iter() {
                    key_value_pairs.push((self.to_guest_any(key)?, self.to_guest_any(value)?));
                }

                let nested_map_access = self.bindings.floria_plugins_dispatch().nested_map();
                let resource = nested_map_access
                    .call_constructor(&mut self.store, &key_value_pairs)
                    .map_err(PluginError::CallWasm)?;
                Ok(dispatch::Any::NestedMap(resource))
            }
        }
    }

    /// Convert a [dispatch::Any] to a [normal::Variant].
    pub fn from_guest_value<AnnotatedT>(
        &mut self,
        value: dispatch::Any,
    ) -> Result<normal::Variant<AnnotatedT>, PluginError>
    where
        AnnotatedT: Default,
    {
        match value {
            dispatch::Any::Null => Ok(normal::Null::default().into()),

            dispatch::Any::Integer(integer) => Ok(normal::Integer::new(integer).into()),

            dispatch::Any::UnsignedInteger(unsigned_integer) => {
                Ok(normal::UnsignedInteger::new(unsigned_integer).into())
            }

            dispatch::Any::Float(float) => Ok(normal::Float::from(float).into()),

            dispatch::Any::Boolean(boolean) => Ok(normal::Boolean::new(boolean).into()),

            dispatch::Any::Text(text) => Ok(normal::Text::from(text).into()),

            dispatch::Any::Blob(blob) => Ok(normal::Blob::from(blob).into()),

            dispatch::Any::NestedList(resource) => {
                let nested_list_access = self.bindings.floria_plugins_dispatch().nested_list();
                let vector = nested_list_access.call_get(&mut self.store, resource).map_err(PluginError::CallWasm)?;
                resource.resource_drop(&mut self.store).map_err(PluginError::CallWasm)?;

                let mut normal_list = Vec::with_capacity(vector.len());
                for value in vector {
                    normal_list.push(self.from_guest_value(value)?);
                }

                Ok(normal_list.into())
            }

            dispatch::Any::NestedMap(resource) => {
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
