use super::{super::store::*, call::*, debug::*, id::*, metadata::*};

use {compris::normal::*, kutil_cli::debug::*, std::collections::*, std::io};

//
// Property
//

/// Property.
///
/// Equivalent to TOSCA property or attribute.
#[derive(Clone, Debug)]
pub struct Property<AnnotatedT> {
    /// Metadata.
    pub metadata: Metadata<AnnotatedT>,

    /// Group IDs.
    pub group_ids: Vec<ID>,

    /// Value.
    pub value: Option<Value<AnnotatedT>>,

    /// Updater.
    pub updater: Option<Call<AnnotatedT>>,

    /// Validator.
    pub validator: Option<Call<AnnotatedT>>,

    /// Read-only.
    pub read_only: bool,
}

impl<AnnotatedT> Property<AnnotatedT> {
    /// Constructor.
    pub fn new(
        value: Option<Value<AnnotatedT>>,
        updater: Option<Call<AnnotatedT>>,
        validator: Option<Call<AnnotatedT>>,
        read_only: bool,
    ) -> Self
    where
        AnnotatedT: Default,
    {
        Self { metadata: Metadata::default(), group_ids: Vec::new(), value, updater, validator, read_only }
    }

    /// To [Value].
    pub fn to_value<'own, StoreT>(&self, embedded: bool, store: &'own StoreT) -> Result<Value<AnnotatedT>, StoreError>
    where
        AnnotatedT: Clone + Default,
        StoreT: Store<AnnotatedT>,
    {
        let mut map = BTreeMap::new();

        map.insert("metadata".into(), self.metadata.clone().into());

        if !self.group_ids.is_empty() {
            if embedded {
                let mut groups = Vec::with_capacity(self.group_ids.len());
                for group_id in &self.group_ids {
                    match store.get_group(group_id)? {
                        Some(group) => groups.push(group.to_value()),
                        None => {}
                    }
                }
                map.insert("groups".into(), groups.into());
            } else {
                let group_ids: Vec<Value<_>> = self.group_ids.iter().map(|id| id.to_string().into()).collect();
                map.insert("group_ids".into(), group_ids.into());
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
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableProperty<'own, StoreT, AnnotatedT>
    where
        StoreT: Store<AnnotatedT>,
    {
        DebuggableProperty { property: self, store }
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        site: &super::super::plugins::Site,
        library: &mut super::super::plugins::Library<StoreT, AnnotatedT>,
        plugin_name: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), super::super::FloriaError>
    where
        AnnotatedT: Clone + Default,
        StoreT: super::super::Store<AnnotatedT>,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::FloriaError>,
    {
        use kutil_std::error::*;
        if let Some(updater) = &self.updater {
            self.value = updater.evaluate(site, library, plugin_name).map(Some).give_or(None, errors)?;
        };
        Ok(())
    }
}

//
// DebuggableProperty
//

/// Debuggable property.
pub struct DebuggableProperty<'own, StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    property: &'own Property<AnnotatedT>,
    store: &'own StoreT,
}

impl<'own, StoreT, AnnotatedT> Debuggable for DebuggableProperty<'own, StoreT, AnnotatedT>
where
    StoreT: Store<AnnotatedT>,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.separate(writer)?;
        context.theme.write_heading(writer, "Property")?;
        write_debug_metadata(&self.property.metadata, false, writer, context)?;
        write_debug_groups(&self.property.group_ids, self.store, writer, context)?;

        utils::write_debug_field("value", false, writer, context, |writer, context| -> io::Result<()> {
            match &self.property.value {
                Some(value) => value.write_debug_for(writer, context),
                None => {
                    context.separate(writer)?;
                    context.theme.write_symbol(writer, "None")
                }
            }
        })?;

        utils::write_debug_field("updater", false, writer, context, |writer, context| -> io::Result<()> {
            match &self.property.updater {
                Some(updater) => updater.write_debug_for(writer, context),
                None => {
                    context.separate(writer)?;
                    context.theme.write_symbol(writer, "None")
                }
            }
        })?;

        utils::write_debug_field("validator", false, writer, context, |writer, context| -> io::Result<()> {
            match &self.property.validator {
                Some(validator) => validator.write_debug_for(writer, context),
                None => {
                    context.separate(writer)?;
                    context.theme.write_symbol(writer, "None")
                }
            }
        })?;

        utils::write_debug_field("read_only", true, writer, context, |writer, context| -> io::Result<()> {
            context.separate(writer)?;
            context.theme.write_symbol(writer, self.property.read_only)
        })
    }
}
