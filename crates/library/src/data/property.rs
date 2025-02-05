use super::{super::store::*, call::*, debug::*, id::*, metadata::*};

use {compris::normal::*, kutil_cli::debug::*, std::collections::*, std::io};

//
// Property
//

/// Property.
///
/// Equivalent to TOSCA property or attribute.
#[derive(Clone, Debug)]
pub struct Property {
    /// Metadata.
    pub metadata: Metadata,

    /// Type IDs.
    pub type_ids: Vec<ID>,

    /// Value.
    pub value: Option<Value>,

    /// Updater.
    pub updater: Option<Call>,

    /// Validator.
    pub validator: Option<Call>,

    /// Read-only.
    pub read_only: bool,
}

impl Property {
    /// Constructor.
    pub fn new(value: Option<Value>, updater: Option<Call>, validator: Option<Call>, read_only: bool) -> Self {
        Self { metadata: Metadata::new(), type_ids: Vec::new(), value, updater, validator, read_only }
    }

    /// To [Value].
    pub fn to_value<'own, StoreT>(&self, embedded: bool, store: &'own StoreT) -> Result<Value, StoreError>
    where
        StoreT: StoreClient,
    {
        let mut map = BTreeMap::new();

        map.insert("metadata".into(), self.metadata.clone().into());

        if !self.type_ids.is_empty() {
            if embedded {
                let mut types = Vec::with_capacity(self.type_ids.len());
                for type_id in &self.type_ids {
                    match store.get_type(type_id)? {
                        Some(type_) => types.push(type_.to_value()),
                        None => {}
                    }
                }
                map.insert("types".into(), types.into());
            } else {
                let type_ids: Vec<Value> = self.type_ids.iter().map(|i| i.to_string().into()).collect();
                map.insert("type_ids".into(), type_ids.into());
            }
        }

        if let Some(value) = &self.value {
            map.insert("value".into(), value.clone());
        }
        if let Some(updater) = &self.updater {
            map.insert("updater".into(), updater.to_value());
        }
        if let Some(validator) = &self.validator {
            map.insert("validator".into(), validator.to_value());
        }
        map.insert("read_only".into(), self.read_only.into());

        Ok(map.into())
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableProperty<'own, StoreT>
    where
        StoreT: StoreClient,
    {
        DebuggableProperty { property: self, store }
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), super::super::ImperativeError>
    where
        StoreT: super::super::StoreClient,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::ImperativeError>,
    {
        use kutil_std::error::*;
        if let Some(updater) = &self.updater {
            self.value = updater.evaluate(site, library, plugin_name).map(|v| Some(v)).give_or(None, errors)?;
        };
        Ok(())
    }
}

//
// DebuggableProperty
//

/// Debuggable property.
pub struct DebuggableProperty<'own, StoreT>
where
    StoreT: StoreClient,
{
    property: &'own Property,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableProperty<'own, StoreT>
where
    StoreT: StoreClient,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.separate(writer)?;
        context.theme.write_heading(writer, "Property")?;
        write_debug_metadata(&self.property.metadata, false, writer, context)?;
        write_debug_types(&self.property.type_ids, self.store, writer, context)?;

        utils::write_debug_field("value", false, writer, context, |writer, context| -> io::Result<()> {
            match &self.property.value {
                Some(value) => value.write_debug_for(writer, context),
                None => {
                    context.separate(writer)?;
                    context.theme.write_bare(writer, "None")
                }
            }
        })?;

        utils::write_debug_field("updater", false, writer, context, |writer, context| -> io::Result<()> {
            match &self.property.updater {
                Some(updater) => updater.write_debug_for(writer, context),
                None => {
                    context.separate(writer)?;
                    context.theme.write_bare(writer, "None")
                }
            }
        })?;

        utils::write_debug_field("validator", false, writer, context, |writer, context| -> io::Result<()> {
            match &self.property.validator {
                Some(validator) => validator.write_debug_for(writer, context),
                None => {
                    context.separate(writer)?;
                    context.theme.write_bare(writer, "None")
                }
            }
        })?;

        utils::write_debug_field("read_only", true, writer, context, |writer, context| -> io::Result<()> {
            context.separate(writer)?;
            context.theme.write_bare(writer, self.property.read_only)
        })
    }
}
