use super::{super::store::*, depict::*, expression::*, id::*, metadata::*};

use {
    compris::{annotate::*, normal::*},
    kutil::cli::depict::*,
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

    /// Class IDs.
    pub class_ids: Vec<ID>,

    /// Value.
    pub value: Option<Variant<WithoutAnnotations>>,

    /// Updater.
    pub updater: Option<Expression>,

    /// Validator.
    pub validator: Option<Expression>,

    /// Read-only.
    pub read_only: bool,
}

impl Property {
    /// Constructor.
    pub fn new(
        value: Option<Variant<WithoutAnnotations>>,
        updater: Option<Expression>,
        validator: Option<Expression>,
        read_only: bool,
    ) -> Self {
        Self { metadata: Default::default(), class_ids: Default::default(), value, updater, validator, read_only }
    }

    /// To Compris variant.
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

        if !self.class_ids.is_empty() {
            if embedded {
                let mut classes = List::new_with_capacity(self.class_ids.len());
                for class_id in &self.class_ids {
                    match store.get_class(class_id)? {
                        Some(class) => classes.inner.push(class.to_variant()),
                        None => {}
                    }
                }
                map.into_insert("classes", classes);
            } else {
                let class_ids: List<_> = self.class_ids.iter().map(|id| id.to_string().into()).collect();
                map.into_insert("class_ids", class_ids);
            }
        }

        if let Some(value) = &self.value {
            map.into_insert("value", value.clone().into_annotated());
        }

        if let Some(updater) = &self.updater {
            map.into_insert("updater", updater);
        }

        if let Some(validator) = &self.validator {
            map.into_insert("validator", validator);
        }

        map.into_insert("read_only", self.read_only);

        Ok(map.into())
    }

    /// To [Depict].
    pub fn to_depict<'own, StoreT>(&'own self, store: &'own StoreT) -> DepictProperty<'own, StoreT>
    where
        StoreT: Store,
    {
        DepictProperty { property: self, store }
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update<StoreT, ErrorRecipientT>(
        &mut self,
        id: &ID,
        property_name: &str,
        library: &mut super::super::plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, super::super::FloriaError>
    where
        StoreT: Clone + Send + super::super::Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        use {super::super::plugins::*, kutil::std::error::*};

        if self.read_only && self.value.is_some() {
            // Read-only properties can only be updated once.
            return Ok(false);
        }

        if let Some(updater) = &self.updater {
            let site = Site::new(id.clone(), vec![property_name.into()]);
            self.value = unwrap_or_give!(updater.evaluate(&site, library).map(Some), errors, None);
            return Ok(true);
        }

        Ok(false)
    }

    /// Validate.
    #[cfg(feature = "plugins")]
    pub fn validate<StoreT, ErrorRecipientT>(
        &mut self,
        id: &ID,
        property_name: &str,
        library: &mut super::super::plugins::Library<StoreT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<bool, super::super::FloriaError>
    where
        StoreT: Clone + Send + super::super::Store,
        ErrorRecipientT: kutil::std::error::ErrorRecipient<super::super::FloriaError>,
    {
        use {
            super::super::{errors::*, plugins::*},
            kutil::std::error::*,
        };

        if let Some(validator) = &self.validator {
            let site = Site::new(id.clone(), vec![property_name.into()]);
            let valid =
                unwrap_or_give!(validator.evaluate::<_, WithoutAnnotations>(&site, library).map(Some), errors, None,);

            let valid = match valid {
                Some(Variant::Boolean(boolean)) => boolean.inner == true,
                _ => false,
            };

            if !valid {
                self.value = None;
                errors.give(InvalidValueError::new(site.id.clone().into(), site.path))?;
                return Ok(false);
            }
        }

        Ok(true)
    }
}

//
// DepictProperty
//

/// Depict property.
pub struct DepictProperty<'own, StoreT>
where
    StoreT: Store,
{
    property: &'own Property,
    store: &'own StoreT,
}

impl<'own, StoreT> Depict for DepictProperty<'own, StoreT>
where
    StoreT: Store,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let context = &context.child().with_separator(true);

        context.separate(writer)?;
        context.theme.write_heading(writer, "Property")?;
        depict_metadata(&self.property.metadata, false, writer, context)?;
        depict_classes(&self.property.class_ids, self.store, writer, context)?;

        utils::depict_field("value", false, writer, context, |writer, context| match &self.property.value {
            Some(value) => value.depict(writer, context),
            None => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "None")
            }
        })?;

        utils::depict_field("updater", false, writer, context, |writer, context| match &self.property.updater {
            Some(updater) => updater.depict(writer, context),
            None => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "None")
            }
        })?;

        utils::depict_field("validator", false, writer, context, |writer, context| match &self.property.validator {
            Some(validator) => validator.depict(writer, context),
            None => {
                context.separate(writer)?;
                context.theme.write_symbol(writer, "None")
            }
        })?;

        utils::depict_field("read_only", true, writer, context, |writer, context| {
            context.separate(writer)?;
            context.theme.write_symbol(writer, self.property.read_only)
        })
    }
}
