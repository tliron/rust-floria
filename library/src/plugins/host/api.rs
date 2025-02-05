use super::{
    super::{
        super::{data::*, store::*},
        bindings::floria::plugins::floria as host,
    },
    host::*,
};

use compris::annotate::*;

impl<StoreT> host::Host for PluginHost<StoreT>
where
    StoreT: Clone + Send + Store,
{
    fn log(&mut self, source: String, message: String) -> wasmtime::Result<()> {
        tracing::info!("[{}] {}: {}", self.name, source, message);
        Ok(())
    }

    fn evaluate_expression(
        &mut self,
        _expression: host::Any,
        site: host::Site,
    ) -> wasmtime::Result<Result<host::Any, String>> {
        // TODO: also need to make sure we're not calling into same plugin
        let expression = Expression::default();
        let any = expression.evaluate::<_, WithAnnotations>(&site.into(), &mut self.library).unwrap();
        let any = self.to_any(any)?;
        Ok(Ok(any))
    }

    fn get_entity(&mut self, id: host::Id) -> wasmtime::Result<Result<host::Any, String>> {
        Ok(match self.library.store.get_entity_as_variant::<WithoutAnnotations>(&id.into())? {
            Some(entity) => Ok(self.to_any(entity)?),
            None => todo!(),
        })
    }
}
