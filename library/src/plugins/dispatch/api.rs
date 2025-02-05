use super::{
    super::{super::store::*, bindings::exports::floria::plugins::dispatch::Site, errors::*},
    error::*,
    plugin::*,
};

use compris::normal::*;

impl<StoreT> DispatchPlugin<StoreT>
where
    StoreT: Store,
{
    /// Dispatch.
    pub fn dispatch<AnnotatedT>(
        &mut self,
        name: &str,
        arguments: Vec<Variant<AnnotatedT>>,
        site: &Site,
    ) -> Result<Variant<AnnotatedT>, PluginError>
    where
        AnnotatedT: Default,
    {
        // TODO: optimize this?
        let length = arguments.len();
        let mut string_arguments = Vec::with_capacity(length);
        let mut dispatch_arguments = Vec::with_capacity(length);
        for argument in arguments.into_iter() {
            string_arguments.push(argument.to_string());
            dispatch_arguments.push(self.to_any(argument)?);
        }

        tracing::debug!("dispatch: {}({}) at {}", name, string_arguments.join(","), site);

        let value = self
            .bindings
            .floria_plugins_dispatch()
            .call_dispatch(&mut self.host, name, &dispatch_arguments, site)
            .map_err(PluginError::CallWasm)?
            .map_err(|error| {
                DispatchError::new(
                    error.to_string(),
                    self.name.to_string(),
                    name.into(),
                    string_arguments,
                    site.clone(),
                )
            })?;

        Ok(self.from_any(value)?)
    }
}
