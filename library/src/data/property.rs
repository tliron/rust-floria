use super::{super::store::*, call::*, debug::*, id::*, metadata::*};

use {
    compris::{annotate::*, normal::*},
    kutil_cli::debug::*,
    std::io,
};

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

    /// Group IDs.
    pub group_ids: Vec<ID>,

    /// Variant.
    pub variant: Option<Variant<WithoutAnnotations>>,

    /// Updater.
    pub updater: Option<Call>,

    /// Validator.
    pub validator: Option<Call>,

    /// Read-only.
    pub read_only: bool,
}

impl Property {
    /// Constructor.
    pub fn new(
        variant: Option<Variant<WithoutAnnotations>>,
        updater: Option<Call>,
        validator: Option<Call>,
        read_only: bool,
    ) -> Self {
        Self { metadata: Metadata::default(), group_ids: Vec::new(), variant, updater, validator, read_only }
    }

    /// To [Variant].
    pub fn to_variant<'own, StoreT, AnnotatedT>(
        &self,
        embedded: bool,
        store: &'own StoreT,
    ) -> Result<Variant<AnnotatedT>, StoreError>
    where
        StoreT: Store,
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = Map::default();

        map.into_insert("metadata", self.metadata.clone().into_annotated());

        if !self.group_ids.is_empty() {
            if embedded {
                let mut groups = List::new_with_capacity(self.group_ids.len());
                for group_id in &self.group_ids {
                    match store.get_group(group_id)? {
                        Some(group) => groups.inner.push(group.to_variant()),
                        None => {}
                    }
                }
                map.into_insert("groups", groups);
            } else {
                let group_ids: List<_> = self.group_ids.iter().map(|id| id.to_string().into()).collect();
                map.into_insert("group_ids", group_ids);
            }
        }

        if let Some(value) = &self.variant {
            map.into_insert("value", value.clone().into_annotated());
        }
        if let Some(updater) = &self.updater {
            map.into_insert("updater", updater.to_variant());
        }
        if let Some(validator) = &self.validator {
            map.into_insert("validator", validator.to_variant());
        }
        map.into_insert("read_only", self.read_only);

        Ok(map.into())
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableProperty<'own, StoreT>
    where
        StoreT: Store,
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
    ) -> Result<(), super::super::FloriaError>
    where
        StoreT: super::super::Store,
        ErrorRecipientT: kutil_std::error::ErrorRecipient<super::super::FloriaError>,
    {
        use kutil_std::error::*;
        if let Some(updater) = &self.updater {
            self.variant = updater.evaluate(site, library, plugin_name).map(Some).give_or(None, errors)?;
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
    StoreT: Store,
{
    property: &'own Property,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableProperty<'own, StoreT>
where
    StoreT: Store,
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
            match &self.property.variant {
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
